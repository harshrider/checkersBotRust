use rayon::prelude::*;
use std::cmp::Ordering;
use crate::board::{Board, Color};
use crate::mv::Move;
use crate::ab_ai::{bar, ab_ai};

pub fn parallel_loss_function(board: &Board, depth: u32) -> Option<Move> {

    //Base Cases
    if depth == 0 || board.is_game_over() {
        return None;
    }
    let valid_moves = board.get_valid_moves();

    if valid_moves.is_empty() {
        return None;
    }
    if depth <= 5 {
        return ab_ai(board, depth);
    }

    if depth == 1 {
        return par_eval(board, &valid_moves);
    }

    valid_moves.par_iter()
        .map(|mv| {
            let mut new_board = board.clone();
            let _ = new_board.make_move(mv);

            let future_best_move = parallel_loss_function(&new_board, depth - 1);

            let score = match future_best_move {
                Some(best_future_move) => {
                    // Apply the future best move to get the resulting position
                    let mut future_board = new_board.clone();
                    let _ = future_board.make_move(&best_future_move);

                    bar(&future_board)
                },
                None => {
                    bar(&new_board)
                }
            };
            (mv.clone(), score)
        })
        .max_by(|a, b| {
            if board.turn == Color::Red {
                a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)
            } else {
                b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
            }
        })
        .map(|(mv, _)| mv)
}

/// Helper function to evaluate all moves in parallel at leaf nodes
fn par_eval(board: &Board, valid_moves: &[Move]) -> Option<Move> {
    valid_moves.par_iter()
        .map(|mv| {
            let mut new_board = board.clone();
            let _ = new_board.make_move(mv);

            let score = bar(&new_board);
            (mv.clone(), score)
        })
        .max_by(|a, b| {
            // Maximize for Red, minimize for Black
            if board.turn == Color::Red {
                a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)
            } else {
                b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
            }
        })
        .map(|(mv, _)| mv)
}
