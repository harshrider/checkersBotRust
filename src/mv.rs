

use crate::board::{Board, Color};

// Represents a move in the game
#[derive(Debug, Clone)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub captures: Vec<usize>,// Can be multiple capture moves
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



    false// Temp
}