use crate::board::{Board, Color};
use std::iter::Iterator;

// Represents a move in the game
#[derive(Debug, Clone)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub captures: Vec<usize>,// Can be multiple capture moves
    pub path: Vec<usize>,    // Sequence of positions for multi-jumps (including from and to)
}

impl Move {
    pub fn new(from: usize, to: usize, captures: Vec<usize>) -> Self {
        Move {
            from,
            to,
            captures,
            path: vec![from, to],
        }
    }

    // Create a move with specified path for multi-jumps
    pub fn with_path(from: usize, to: usize, captures: Vec<usize>, path: Vec<usize>) -> Self {
        Move {
            from,
            to,
            captures,
            path,
        }
    }

    // Algebraic notation (e.g., "E3-F4" or "B3-D5-F3" for multi-capture)
    pub fn to_notation(&self, board: &Board) -> String {
        if self.path.len() > 2 {
            // Functional approach for multi-jump notation
            self.path.iter()
                .enumerate()
                .map(|(i, &position)| {
                    let (row, col) = board.index_to_coords(position);
                    let notation = format!("{}{}", (col as u8 + b'A') as char, row + 1);
                    if i > 0 { format!("-{}", notation) } else { notation }
                })
                .collect::<String>()
        } else {
            // Simple move
            let (from_row, from_col) = board.index_to_coords(self.from);
            let (to_row, to_col) = board.index_to_coords(self.to);

            format!(
                "{}{}-{}{}",
                (from_col as u8 + b'A') as char, from_row + 1,
                (to_col as u8 + b'A') as char, to_row + 1
            )
        }
    }

    pub fn from_notation(pos: &str, board: &Board) -> Option<Self> {
        //println!("Parsing : '{}'", pos);

        let parts: Vec<&str> = pos.split('-').collect();
        //println!("parts: {:?}", parts);

        if parts.len() < 2 {
            //println!("Invalid format: Need at least a source and destination");
            return None;
        }

        // Convert all positions in the path to board indices using map
        let positions_result: Option<Vec<((usize, usize), usize)>> = parts.iter()
            .map(|&notation| {
                // Validate and parse a single notation like "E3"
                if notation.len() < 2 {
                    //println!("Notation part too short: {}", notation);
                    return None;
                }

                let chars: Vec<char> = notation.chars().collect();
                let col_char = chars[0];
                let row_char = chars[1];

                if !col_char.is_ascii_alphabetic() || !row_char.is_ascii_digit() {
                    //println!("Invalid format in position: {}", notation);
                    return None;
                }

                let col = (col_char.to_ascii_uppercase() as u8 - b'A') as usize;
                let row = row_char.to_digit(10).map(|d| d as usize - 1)?;

                //println!("Position: ({}, {})", row, col);

                if row >= 8 || col >= 8 {
                    //println!("Position out of bounds: {}", notation);
                    return None;
                }

                // Get board index from coordinates
                let index = board.coords_to_index(row, col)?;

                Some(((row, col), index))
            })
            .collect();

        // Early return if any position parsing failed
        let positions = match positions_result {
            Some(p) => p,
            None => return None,
        };

        // Extract indices for path
        let indices: Vec<usize> = positions.iter().map(|&(_, index)| index).collect();

        // First position is the start, last is the end
        let from_index = indices[0];
        let to_index = indices[indices.len() - 1];

        // Calculate captures for multi-jumps
        let captures = if indices.len() > 2 {
            // Process consecutive position pairs to find captures
            positions.windows(2)
                .filter_map(|window| {
                    let ((start_row, start_col), _) = window[0];
                    let ((end_row, end_col), _) = window[1];

                    // Calculate the middle position (captured piece)
                    let middle_row = (start_row + end_row) / 2;
                    let middle_col = (start_col + end_col) / 2;

                    // Check if it's a valid capture (diagonal and distance of 2)
                    let row_diff = (start_row as i32 - end_row as i32).abs();
                    let col_diff = (start_col as i32 - end_col as i32).abs();

                    if row_diff != 2 || col_diff != 2 {
                        //println!("Invalid jump distance between positions");
                        return None;
                    }

                    let middle_index = board.coords_to_index(middle_row, middle_col)?;
                    //println!("Capture at: ({}, {}) -> index {}", middle_row, middle_col, middle_index);

                    Some(middle_index)
                })
                .collect()
        } else {
            // For simple jumps, check if it's a capture based on distance
            let ((start_row, start_col), _) = positions[0];
            let ((end_row, end_col), _) = positions[1];

            let row_diff = (start_row as i32 - end_row as i32).abs();
            let col_diff = (start_col as i32 - end_col as i32).abs();

            // If the distance is 2, it's a capture
            if row_diff == 2 && col_diff == 2 {
                let middle_row = (start_row + end_row) / 2;
                let middle_col = (start_col + end_col) / 2;

                board.coords_to_index(middle_row, middle_col)
                    .map(|middle_index| {
                        //println!("Middle index: {}, Piece: '{}'", middle_index, board.squares[middle_index]);
                        vec![middle_index]
                    })
                    .unwrap_or_else(|| {
                        //println!("No valid middle index for capture");
                        vec![]
                    })
            } else {
                vec![]
            }
        };

        //println!("Final move - From: {}, To: {}, Path: {:?}, Captures: {:?}",from_index, to_index, indices, captures);

        Some(Move {
            from: from_index,
            to: to_index,
            captures,
            path: indices,
        })
    }
}

// The definitive function to check if a move is valid according to checkers rules
pub fn is_valid_move(board: &Board, m: &Move) -> bool {
    //println!("Validating move: From {} to {}, Path: {:?}, Captures: {:?}", m.from, m.to, m.path, m.captures);

    // Basic validation checks
    if m.from >= 32 || m.to >= 32 {
        //println!("Invalid indices");
        return false;
    }

    let piece = board.squares[m.from];
    //println!("Piece at source: '{}'", piece);

    // Check if piece belongs to current player using pattern matching
    let piece_belongs_to_current_player = match (board.turn, piece) {
        (Color::Red, 'r') | (Color::Red, 'R') => true,
        (Color::Black, 'b') | (Color::Black, 'B') => true,
        _ => false
    };

    if !piece_belongs_to_current_player {
        //println!("Piece doesn't belong to current player");
        return false;
    }

    // Check if destination is empty
    if board.squares[m.to] != '□' {
        //println!("Destination not empty");
        return false;
    }

    let is_king = piece == 'R' || piece == 'B';

    // For multi-jumps, validate each segment of the path
    if m.path.len() > 2 {
        //println!("Validating multi-jump path with {} segments", m.path.len() - 1);

        // Each captured piece must belong to the opponent - use all() to check
        let all_captures_valid = m.captures.iter().all(|&capture_index| {
            let captured_piece = board.squares[capture_index];
            //println!("Checking captured piece at index {}: '{}'", capture_index, captured_piece);

            let is_valid_capture = match board.turn {
                Color::Red => captured_piece == 'b' || captured_piece == 'B',
                Color::Black => captured_piece == 'r' || captured_piece == 'R',
            };

            if !is_valid_capture {
                //println!("Invalid capture: not an opponent's piece");
            }

            is_valid_capture
        });

        if !all_captures_valid {
            return false;
        }

        // Check each segment of the path using windows() and all()
        let all_segments_valid = m.path.windows(2).all(|window| {
            let from_pos = window[0];
            let to_pos = window[1];

            let (from_row, from_col) = board.index_to_coords(from_pos);
            let (to_row, to_col) = board.index_to_coords(to_pos);

            let row_diff = (from_row as i32 - to_row as i32).abs();
            let col_diff = (from_col as i32 - to_col as i32).abs();

            //println!("Checking jump segment {} -> {}: row diff = {}, col diff = {}", from_pos, to_pos, row_diff, col_diff);

            // Each jump must be diagonal
            if row_diff != col_diff {
                //println!("Jump segment not diagonal");
                return false;
            }

            // Each jump must be 2 squares (capture)
            if row_diff != 2 {
                //println!("Jump segment distance not 2");
                return false;
            }

            // For regular pieces, check direction
            if !is_king {
                match piece {
                    'r' if from_row <= to_row => {
                        //println!("Red pawn can only move/capture up");
                        return false;
                    },
                    'b' if from_row >= to_row => {
                        //println!("Black pawn can only move/capture down");
                        return false;
                    },
                    _ => {}
                }
            }

            true
        });

        if !all_segments_valid {
            return false;
        }

        // The number of captures = jumps
        if m.captures.len() != m.path.len() - 1 {
            //println!("Capture count doesn't match jump count");
            return false;
        }

        return true;
    }

    // Simple move (non-multi-jump)
    let (from_row, from_col) = board.index_to_coords(m.from);
    let (to_row, to_col) = board.index_to_coords(m.to);

    //println!("From: ({}, {}), To: ({}, {})", from_row, from_col, to_row, to_col);

    let row_diff = (from_row as i32 - to_row as i32).abs();
    let col_diff = (from_col as i32 - to_col as i32).abs();

    //println!("Row diff: {}, Col diff: {}", row_diff, col_diff);

    // Check if move is diagonal
    if row_diff != col_diff {
        //println!("Not diagonal");
        return false;
    }

    // Regular move
    if m.captures.is_empty() {
        // Check if it's only one square
        if row_diff != 1 {
            //println!("Regular move must be one square");
            return false;
        }

        // Direction check for pawns with pattern matching
        if !is_king {
            match (piece, from_row.cmp(&to_row)) {
                ('r', std::cmp::Ordering::Less | std::cmp::Ordering::Equal) => {
                    //println!("Red pawn can only move up");
                    return false;
                },
                ('b', std::cmp::Ordering::Greater | std::cmp::Ordering::Equal) => {
                    //println!("Black pawn can only move down");
                    return false;
                },
                _ => {}
            }
        }
    }
    // Capture move (single jump)
    else {
        if row_diff != 2 {
            //println!("Capture jump distance must be 2");
            return false;
        }

        // Use all() to check all captures are valid
        let all_captures_valid = m.captures.iter().all(|&capture_index| {
            let captured_piece = board.squares[capture_index];
            //println!("Validating capture at index {}, piece: '{}'", capture_index, captured_piece);

            match (board.turn, captured_piece) {
                (Color::Red, 'b') | (Color::Red, 'B') => true,
                (Color::Black, 'r') | (Color::Black, 'R') => true,
                _ => {
                    //println!("Not a valid piece to capture");
                    false
                }
            }
        });

        if !all_captures_valid {
            return false;
        }

        // Direction check for pawns with pattern matching
        if !is_king {
            match (piece, from_row.cmp(&to_row)) {
                ('r', std::cmp::Ordering::Less | std::cmp::Ordering::Equal) => {
                    //println!("Red pawn can only capture up");
                    return false;
                },
                ('b', std::cmp::Ordering::Greater | std::cmp::Ordering::Equal) => {
                    //println!("Black pawn can only capture down");
                    return false;
                },
                _ => {}
            }
        }
    }


    //println!("Move is valid");
    true
}

// Function to determine if a piece should be promoted to a king
pub fn promote(board: &Board, index: usize) -> bool {
    // Use pattern matching to check for promotion conditions
    match (board.squares[index], board.index_to_coords(index)) {
        ('r', (0, _)) => {
            //println!("Promoting red piece to king");
            true
        },
        ('b', (7, _)) => {
            //println!("Promoting black piece to king");
            true
        },
        _ => false
    }
}