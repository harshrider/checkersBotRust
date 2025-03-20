

use crate::board::{Board, Color};

// Represents a move in the game
#[derive(Debug, Clone)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub captures: Vec<usize>,
}


pub fn is_valid_move(board: &Board, m: &Move) -> bool {
    // Check if the move is in the board
    if m.from >= 32 || m.to >= 32 {
        return false;
    }

    // Check if the source square has a piece of the current player's color
    let piece = board.squares[m.from];
    match board.turn {
        Color::Red => if piece != 'r' && piece != 'R' { return false; },
        Color::Black => if piece != 'b' && piece != 'B' { return false; },
    }

    // Check if the destination square is empty
    if board.squares[m.to] != '□' {
        return false;
    }

    // Calculate move direction
    let (from_row, from_col) = board.index_to_coords(m.from);
    let (to_row, to_col) = board.index_to_coords(m.to);

    // Check if the move is diagonal
    if (from_row as i32 - to_row as i32).abs() != (from_col as i32 - to_col as i32).abs() {
        return false;
    }
    return false;// Temp
}