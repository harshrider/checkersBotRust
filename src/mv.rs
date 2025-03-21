

use crate::board::{Board, Color};

// Represents a move in the game
#[derive(Debug, Clone)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub captures: Vec<usize>,// Can be multiple capture moves
}


impl Move {

    pub fn new(from: usize, to: usize, captures: Vec<usize>) -> Self {
        Move {
            from,
            to,
            captures,
        }
    }

    // Algebraic notation (e.g., "E3-F4" or "E3xG5") - to move x to capture
    pub fn to_notation(&self, board: &Board) -> String {
        let (from_row, from_col) = board.index_to_coords(self.from);
        let (to_row, to_col) = board.index_to_coords(self.to);

        let from_notation = format!("{}{}", (from_col as u8 + b'A') as char, from_row + 1);
        let to_notation = format!("{}{}", (to_col as u8 + b'A') as char, to_row + 1);

        if self.captures.is_empty() {
            format!("{}-{}", from_notation, to_notation)
        } else {
            format!("{}x{}", from_notation, to_notation)
        }
    }
}

pub fn is_valid_move(board: &Board, m: &Move) -> bool {
    // Basic Move Validation
    if m.from >= 32 || m.to >= 32 {
        return false;
    }
    let piece = board.squares[m.from];
    match board.turn {
        Color::Red => if piece != 'r' && piece != 'R' { return false; },
        Color::Black => if piece != 'b' && piece != 'B' { return false; },
    }
    if board.squares[m.to] != '□' {
        return false;
    }
    let (from_row, from_col) = board.index_to_coords(m.from);
    let (to_row, to_col) = board.index_to_coords(m.to);

    if (from_row as i32 - to_row as i32).abs() != (from_col as i32 - to_col as i32).abs() {
        return false;
    }

    // Regular move (pawns)
    if m.captures.is_empty() {
        if (from_row as i32 - to_row as i32).abs() != 1 {
            return false;
        }

        if piece == 'r' && from_row <= to_row {
            return false; // Red pawns can only move up
        }
        if piece == 'b' && from_row >= to_row {
            return false; // Black pawns can only move down
        }
    }


    else {
        if (from_row as i32 - to_row as i32).abs() != 2 {
            return false;
        }
        let middle_row = (from_row + to_row) / 2;
        let middle_col = (from_col + to_col) / 2;

        if let Some(middle_index) = board.coords_to_index(middle_row, middle_col) {
            let middle_piece = board.squares[middle_index];

            match board.turn {
                Color::Red => if middle_piece != 'b' && middle_piece != 'B' { return false; },
                Color::Black => if middle_piece != 'r' && middle_piece != 'R' { return false; },
            }

            if piece == 'r' && from_row <= to_row {
                return false;
            }
            if piece == 'b' && from_row >= to_row {
                return false;
            }
        } else {
            return false;
        }

    }



    true
}