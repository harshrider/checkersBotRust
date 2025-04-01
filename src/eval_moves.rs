use crate::board::{Board, Color};
use crate::mv::{Move, is_valid_move};
use rayon::prelude::*;

pub struct MoveEvaluator {
    pub board: Board,
}

impl MoveEvaluator {
    pub fn new(board: Board) -> Self {
        MoveEvaluator { board }
    }

    fn curr_piece(&self, piece: char) -> bool {
        match self.board.turn {
            Color::Red => piece == 'r' || piece == 'R',
            Color::Black => piece == 'b' || piece == 'B',
        }
    }

    // Generate potential capture moves for a piece
    fn cap_moves(&self, index: usize) -> Vec<Move> {
        let mut potential_captures = Vec::new();
        let piece = self.board.squares[index];

        if !self.curr_piece(piece) {
            return potential_captures;
        }

        let (row, col) = self.board.index_to_coords(index);

        // Direction vectors To check around the peice
        let dir = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        for &(dr, dc) in &dir {
            // For capture moves (2 squares)
            let capture_row = (row as i32) + dr;
            let capture_col = (col as i32) + dc;
            let land_row = (row as i32) + 2 * dr;
            let land_col = (col as i32) + 2 * dc;

            if let (Some(capture_idx), Some(land_idx)) = (
                self.board.coords_to_index(capture_row, capture_col),
                self.board.coords_to_index(land_row, land_col)
            ) {
                let capture_move = Move::new(index, land_idx, vec![capture_idx]);
                if is_valid_move(&self.board, &capture_move) {
                    potential_captures.push(capture_move);
                }
            }
        }

        potential_captures
    }

    // Generate potential regular
    fn reg_moves(&self, index: usize) -> Vec<Move> {
        let mut potential_moves = Vec::new();
        let piece = self.board.squares[index];

        if !self.curr_piece(piece) {
            return potential_moves;
        }

        let (row, col) = self.board.index_to_coords(index);

        let dir = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        for &(dr, dc) in &dir {
            let target_row = (row as i32) + dr;
            let target_col = (col as i32) + dc;

            if let Some(target_idx) = self.board.coords_to_index(target_row, target_col) {
                let regular_move = Move::new(index, target_idx, Vec::new());
                if is_valid_move(&self.board, &regular_move) {
                    potential_moves.push(regular_move);
                }
            }
        }

        potential_moves
    }

    // Generate and check potential multi-capture moves
    fn find_mult_cap(&self, from_idx: usize, visited: &mut Vec<usize>,
                     current_path: &mut Vec<usize>, current_captures: &mut Vec<usize>,
                     all_moves: &mut Vec<Move>) {
        let (row, col) = self.board.index_to_coords(from_idx);

        // Direction vectors (row_delta, col_delta) for all 4 diagonal directions
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        for &(dr, dc) in &directions {
            // Check potential capture in this direction
            let capture_row = (row as i32) + dr;
            let capture_col = (col as i32) + dc;
            let land_row = (row as i32) + 2 * dr;
            let land_col = (col as i32) + 2 * dc;

            if let (Some(capture_idx), Some(land_idx)) = (
                self.board.coords_to_index(capture_row, capture_col),
                self.board.coords_to_index(land_row, land_col)
            ) {
                // Skip if we've already visited this square or captured this piece
                if visited.contains(&land_idx) || current_captures.contains(&capture_idx) {
                    continue;
                }

                // Create move and check if it's valid
                let mut new_path = current_path.clone();
                new_path.push(land_idx);

                let mut new_captures = current_captures.clone();
                new_captures.push(capture_idx);

                let multi_move = Move::with_path(
                    current_path[0],  // Starting position
                    land_idx,         // Current landing position
                    new_captures.clone(),
                    new_path.clone()
                );

                // Check if this move would be valid
                if is_valid_move(&self.board, &multi_move) {
                    // Valid move found, add to list
                    all_moves.push(multi_move);

                    // Continue searching for more captures from new position
                    let mut new_visited = visited.clone();
                    new_visited.push(land_idx);

                    self.find_mult_cap(land_idx, &mut new_visited, &mut new_path,
                                       &mut new_captures, all_moves);
                }
            }
        }
    }

    // Sequential implementation to calculate all valid moves
    pub fn seq_possible_moves(&self) -> Vec<Move> {
        // Start by checking for multi-captures
        let mut mult_cap = Vec::new();

        for i in 0..32 {
            if !self.curr_piece(self.board.squares[i]) {
                continue;
            }

            let mut visited = vec![i];
            let mut current_path = vec![i];
            let mut curr_cap = Vec::new();

            self.find_mult_cap(i, &mut visited, &mut current_path,
                               &mut curr_cap, &mut mult_cap);
        }

        if !mult_cap.is_empty() {
            return mult_cap;
        }

        // If no multi-captures, check for single captures
        let mut all_cap = Vec::new();

        for i in 0..32 {
            let captures = self.cap_moves(i);
            all_cap.extend(captures);
        }

        if !all_cap.is_empty() {
            return all_cap;
        }

        // Only if no captures are available, generate regular moves
        let mut reg_moves = Vec::new();

        for i in 0..32 {
            let moves = self.reg_moves(i);
            reg_moves.extend(moves);
        }

        reg_moves
    }
    // Parallel implementation to calculate all valid moves
    pub fn par_possible_moves(&self) -> Vec<Move> {
        // Start by checking for multi-captures (sequential due to recursive nature)
        let mut mult_cap = Vec::new();

        for i in 0..32 {
            if !self.curr_piece(self.board.squares[i]) {
                continue;
            }

            let mut visited = vec![i];
            let mut current_path = vec![i];
            let mut current_captures = Vec::new();

            self.find_mult_cap(i, &mut visited, &mut current_path,
                               &mut current_captures, &mut mult_cap);
        }

        if !mult_cap.is_empty() {
            return mult_cap;
        }

        // If no multi-captures, check for single captures in parallel
        let all_indices: Vec<usize> = (0..32).collect();

        let all_captures: Vec<Move> = all_indices.par_iter()
            .map(|&idx| self.cap_moves(idx))
            .flatten()
            .collect();

        if !all_captures.is_empty() {
            return all_captures;
        }

        // Only if no captures are available, generate regular moves in parallel
        let all_regular_moves: Vec<Move> = all_indices.par_iter()
            .map(|&idx| self.reg_moves(idx))
            .flatten()
            .collect();

        all_regular_moves
    }

    // pub fn benchmark_performance(&self) -> (std::time::Duration, std::time::Duration) {
    //     use std::time::Instant;
    //
    //     // Benchmark sequential implementation
    //     let seq_start = Instant::now();
    //     let _seq_moves = self.seq_possible_moves();
    //     let seq_duration = seq_start.elapsed();
    //
    //     // Benchmark parallel implementation
    //     let par_start = Instant::now();
    //     let _par_moves = self.par_possible_moves();
    //     let par_duration = par_start.elapsed();
    //
    //     (seq_duration, par_duration)
    // }
}