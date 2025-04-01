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

    pub fn from_notation(pos: &str, board: &Board) -> Option<Self> {
        println!("Parsing : '{}'", pos);

        // Basic Checking
        if !pos.contains('-') && !pos.contains('x') {
            println!(" Missing type of move");
            return None;
        }

        let parts: Vec<&str> = if pos.contains('-') {
            pos.split('-').collect()
        } else {
            pos.split('x').collect()
        };

        println!("parts: {:?}", parts);

        if parts.len() != 2 { // Base Case TODO: This needs to change as captures can have more parts
            println!("Invalid number of parts: {}", parts.len());
            return None;
        }

        let from_notation = parts[0];
        let to_notation = parts[1];

        println!("From: '{}', To: '{}'", from_notation, to_notation);

        if from_notation.len() < 2 || to_notation.len() < 2 {
            println!("Notation parts too short");
            return None;
        }

        // Parse source position
        let from_char = from_notation.chars().next().unwrap();
        let from_num_char = from_notation.chars().nth(1).unwrap();

        println!("From char: '{}', From num: '{}'", from_char, from_num_char);

        if !from_char.is_ascii_alphabetic() || !from_num_char.is_ascii_digit() {
            return None;
        }

        // Parse destination position
        let to_char = to_notation.chars().next().unwrap();
        let to_num_char = to_notation.chars().nth(1).unwrap();

        println!("To char: '{}', To num: '{}'", to_char, to_num_char);

        if !to_char.is_ascii_alphabetic() || !to_num_char.is_ascii_digit() {
            println!("Invalid to notation format");
            return None;
        }

        // Convert to uppercase to handle both upper and lowercase input
        let from_col = (from_char.to_ascii_uppercase() as u8 - b'A') as usize;
        let from_row = from_num_char.to_digit(10).unwrap() as usize - 1;

        let to_col = (to_char.to_ascii_uppercase() as u8 - b'A') as usize;
        let to_row = to_num_char.to_digit(10).unwrap() as usize - 1;

        println!("From ({}, {}), To ({}, {})", from_row, from_col, to_row, to_col);

        // Check if coordinates are within bounds
        if from_row >= 8 || from_col >= 8 || to_row >= 8 || to_col >= 8 {
            println!("Out of bounds");
            return None;
        }

        // Convert to board indices
        let from_index_opt = board.coords_to_index(from_row, from_col);
        let to_index_opt = board.coords_to_index(to_row, to_col);

        println!("From index option: {:?}, To index option: {:?}", from_index_opt, to_index_opt);

        if from_index_opt.is_none() || to_index_opt.is_none() {
            println!("Invalid board index conversion");
            return None;
        }

        let from_index = from_index_opt.unwrap();
        let to_index = to_index_opt.unwrap();

        println!("From index: {}, To index: {}", from_index, to_index);

        // Implement capture detection
        let captures = if pos.contains('x') {
            // Calculate the middle position for a capture
            let middle_row = (from_row + to_row) / 2;
            let middle_col = (from_col + to_col) / 2;

            println!("Middle coords: ({}, {})", middle_row, middle_col);

            if let Some(middle_index) = board.coords_to_index(middle_row, middle_col) {
                println!("Middle index: {}, Piece: '{}'", middle_index, board.squares[middle_index]);
                vec![middle_index]
            } else {
                println!(" No valid middle index for capture");
                Vec::new()
            }
        } else {
            Vec::new()
        };

        println!(" Final move - From: {}, To: {}, Captures: {:?}", from_index, to_index, captures);

        Some(Move {
            from: from_index,
            to: to_index,
            captures,
        })
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

        // Kings can move in any direction, so no direction check needed for 'R' and 'B'
    }
    // Capture move
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

            if !m.captures.contains(&middle_index) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

// Check if a piece should be promoted to a king
pub fn promote(board: &Board, index: usize) -> bool {
    let piece = board.squares[index];
    let (row, _) = board.index_to_coords(index);

    if piece == 'r' && row == 0 {
        return true;
    }

    if piece == 'b' && row == 7 {
        return true;
    }

    false
}