use crate::board::{Board, Color};

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

    // "E3-F4" or "B3-D5-F3" capture)
    pub fn to_notation(&self, board: &Board) -> String {
        let (from_row, from_col) = board.index_to_coords(self.from);
        let (to_row, to_col) = board.index_to_coords(self.to);

        let from_notation = format!("{}{}", (from_col as u8 + b'A') as char, from_row + 1);
        let to_notation = format!("{}{}", (to_col as u8 + b'A') as char, to_row + 1);

        format!("{}-{}", from_notation, to_notation)
    }

    pub fn from_notation(pos: &str, board: &Board) -> Option<Self> {
        println!("Parsing : '{}'", pos);

        let parts: Vec<&str> = pos.split('-').collect();
        println!("parts: {:?}", parts);

        if parts.len() < 2 {
            println!("Invalid format: Needs more than 1");
            return None;
        }

        let from_notation = parts[0];
        let to_notation = parts[parts.len() - 1];

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

        // Determine if this is a capture move based on distance
        let (from_row, from_col) = board.index_to_coords(from_index);
        let (to_row, to_col) = board.index_to_coords(to_index);

        let row_diff = (from_row as i32 - to_row as i32).abs();
        let col_diff = (from_col as i32 - to_col as i32).abs();

        let mut captures = Vec::new();

        // For captures (jumps of 2 or more spaces)
        if row_diff >= 2 && col_diff >= 2 {
            // If we have multiple parts, parse the intermediate jumps
            if parts.len() > 2 {
                // Track current position for multi-capture
                let mut current_row = from_row;
                let mut current_col = from_col;

                // Process each jump
                for i in 1..parts.len() {
                    let jump_notation = parts[i];
                    if jump_notation.len() < 2 {
                        println!("Invalid jump notation");
                        return None;
                    }

                    let jump_char = jump_notation.chars().next().unwrap();
                    let jump_num_char = jump_notation.chars().nth(1).unwrap();

                    if !jump_char.is_ascii_alphabetic() || !jump_num_char.is_ascii_digit() {
                        println!("Invalid jump format");
                        return None;
                    }

                    let jump_col = (jump_char.to_ascii_uppercase() as u8 - b'A') as usize;
                    let jump_row = jump_num_char.to_digit(10).unwrap() as usize - 1;

                    // Calculate and add the captured piece for this jump
                    let middle_row = (current_row + jump_row) / 2;
                    let middle_col = (current_col + jump_col) / 2;

                    if let Some(middle_index) = board.coords_to_index(middle_row, middle_col) {
                        println!("Capture at: ({}, {}) -> index {}", middle_row, middle_col, middle_index);
                        captures.push(middle_index);
                    } else {
                        println!("Invalid middle position for capture");
                        return None;
                    }

                    // Update current position for next jump
                    current_row = jump_row;
                    current_col = jump_col;
                }
            } else {
                // Simple single capture
                let middle_row = (from_row + to_row) / 2;
                let middle_col = (from_col + to_col) / 2;

                if let Some(middle_index) = board.coords_to_index(middle_row, middle_col) {
                    println!("Middle index: {}, Piece: '{}'", middle_index, board.squares[middle_index]);
                    captures.push(middle_index);
                } else {
                    println!("No valid middle index for capture");
                    return None;
                }
            }
        }

        println!("Final move - From: {}, To: {}, Captures: {:?}", from_index, to_index, captures);

        Some(Move {
            from: from_index,
            to: to_index,
            captures,
        })
    }
}

pub fn is_valid_move(board: &Board, m: &Move) -> bool {
    println!("Validating move: From {} to {}, Captures: {:?}", m.from, m.to, m.captures);

    // Basic Move Validation
    if m.from >= 32 || m.to >= 32 {
        println!("Invalid indices");
        return false;
    }

    let piece = board.squares[m.from];
    println!("Piece at source: '{}'", piece);

    // Check if piece belongs to current player
    match board.turn {
        Color::Red => if piece != 'r' && piece != 'R' {
            println!("Not red's piece");
            return false;
        },
        Color::Black => if piece != 'b' && piece != 'B' {
            println!("Not black's piece");
            return false;
        },
    }

    // Check if destination is empty
    if board.squares[m.to] != '□' {
        println!("Destination not empty");
        return false;
    }

    let (from_row, from_col) = board.index_to_coords(m.from);
    let (to_row, to_col) = board.index_to_coords(m.to);

    println!("From: ({}, {}), To: ({}, {})", from_row, from_col, to_row, to_col);

    let row_diff = (from_row as i32 - to_row as i32).abs();
    let col_diff = (from_col as i32 - to_col as i32).abs();

    println!("Row diff: {}, Col diff: {}", row_diff, col_diff);

    // Check if move is diagonal
    if row_diff != col_diff {
        println!("Not diagonal");
        return false;
    }

    let is_king = piece == 'R' || piece == 'B';

    // Regular move (no capture)
    if m.captures.is_empty() {
        // Check if it's only one square
        if row_diff != 1 {
            println!("Regular move must be one square");
            return false;
        }

        // Direction check for pawns (not kings)
        if !is_king {
            if piece == 'r' && from_row <= to_row {
                println!("Red pawn can only move up");
                return false; // Red pawns can only move up
            }
            if piece == 'b' && from_row >= to_row {
                println!("Black pawn can only move down");
                return false; // Black pawns can only move down
            }
        }
    }
    // Capture move
    else {
        // Validate each captured piece
        for &capture_index in &m.captures {
            let captured_piece = board.squares[capture_index];
            println!("Validating capture at index {}, piece: '{}'", capture_index, captured_piece);

            // Check if it's an opponent's piece
            let is_valid_capture = match board.turn {
                Color::Red => captured_piece == 'b' || captured_piece == 'B',
                Color::Black => captured_piece == 'r' || captured_piece == 'R',
            };

            if !is_valid_capture {
                println!("Not a valid piece to capture");
                return false;
            }
        }

        // For simple captures, check the jump distance
        if m.captures.len() == 1 && row_diff != 2 {
            println!("Capture jump distance must be 2");
            return false;
        }

        // Direction check for pawns (not kings)
        if !is_king {
            if piece == 'r' && from_row <= to_row {
                println!("Red pawn can only capture up");
                return false;
            }
            if piece == 'b' && from_row >= to_row {
                println!("Black pawn can only capture down");
                return false;
            }
        }
    }

    println!("Move is valid");
    true
}

// Check if a piece should be promoted to a king
pub fn promote(board: &Board, index: usize) -> bool {
    let piece = board.squares[index];
    let (row, _) = board.index_to_coords(index);

    if piece == 'r' && row == 0 {
        println!("Promoting red piece to king");
        return true;
    }

    if piece == 'b' && row == 7 {
        println!("Promoting black piece to king");
        return true;
    }

    false
}