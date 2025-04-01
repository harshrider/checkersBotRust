use crate::mv::Move;
use std::iter;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Board {
    pub squares: [char; 32],
    pub turn: Color,
    pub red_pieces: u8,
    pub black_pieces: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Black,
}

impl Color {
    // Functional way to toggle turn
    fn toggle(&self) -> Self {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}

impl Board {
    // Create a new board with the initial setup
    pub fn new() -> Self {
        // Use iterators and collect to create the initial squares array
        let squares = (0..32)
            .map(|i| match i {
                0..=11 => 'b',        // black pieces
                20..=31 => 'r',       // red pieces
                _ => '□',             // empty squares
            })
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();  // Safe because we know it's exactly 32 elements

        Board {
            squares,
            turn: Color::Red, // Red goes first in American checkers
            red_pieces: 12,
            black_pieces: 12,
        }
    }

    pub fn index_to_coords(&self, index: usize) -> (usize, usize) {
        let row_group = index / 8;
        let pos_in_group = index % 8;

        let row = if pos_in_group < 4 {
            row_group * 2
        } else {
            row_group * 2 + 1
        };

        let col = if row % 2 == 0 {
            (pos_in_group % 4) * 2 + 1
        } else {
            (pos_in_group % 4) * 2
        };

        (row, col)
    }

    pub fn coords_to_index(&self, row: usize, col: usize) -> Option<usize> {
        // Use pattern matching for early returns
        match (row, col) {
            (r, c) if r >= 8 || c >= 8 => None,
            (r, c) if (r + c) % 2 == 0 => None, // Light squares are not used
            _ => {
                let index = if row % 2 == 0 {
                    // Even row (0, 2, 4, 6) - valid squares are at odd columns
                    (row / 2) * 8 + (col / 2)
                } else {
                    // Odd row (1, 3, 5, 7) - valid squares are at even columns
                    (row / 2) * 8 + 4 + (col / 2)
                };

                if index < 32 { Some(index) } else { None }
            }
        }
    }

    pub fn display(&self) {
        println!("\tA\tB\tC\tD\tE\tF\tG\tH");
        println!("\t_____\t_____\t_____\t_____\t_____\t_____\t_____\t_____");

        // Use iterators for display rows
        (0..8).for_each(|row| {
            print!("{}|\t", row + 1);

            // Use iterators for columns within row
            (0..8).for_each(|col| {
                let piece = if (row + col) % 2 == 1 {
                    self.coords_to_index(row, col)
                        .map(|idx| self.squares[idx])
                        .unwrap_or(' ')
                } else {
                    '■'
                };

                print!("|{}|\t", piece);
            });

            println!();
        });

        println!("Turn: {:?}", self.turn);
        println!("Red pieces: {}, Black pieces: {}", self.red_pieces, self.black_pieces);
    }

    // Check if the game is over - for now, only when all pieces are captured
    pub fn is_game_over(&self) -> bool {
        self.red_pieces == 0 || self.black_pieces == 0
    }

    pub fn get_winner(&self) -> Option<Color> {
        match (self.red_pieces, self.black_pieces) {
            (0, _) => Some(Color::Black),
            (_, 0) => Some(Color::Red),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn get_valid_moves(&self) -> Vec<Move> {
        // This will be implemented in a separate file later
        Vec::new()
    }

    // Execute a move on the board
    pub fn make_move(&mut self, m: &crate::mv::Move) -> Result<(), &'static str> {
        // Get the piece from the source position
        let piece = self.squares[m.from];

        // Create a new board with the piece moved
        if m.path.len() > 2 {
            // For multi-jumps

            // Remove piece from starting position
            self.squares[m.from] = '□';

            // Process each jump segment using iterators
            m.path.iter().enumerate().skip(1).for_each(|(i, &to_pos)| {
                // Place piece at destination
                if i == m.path.len() - 1 {
                    // Final destination
                    self.squares[to_pos] = piece;
                } else {
                    // Intermediate position - place and then clear for next jump
                    self.squares[to_pos] = piece;
                    self.squares[to_pos] = '□';
                }

                // Handle captures
                if i - 1 < m.captures.len() {
                    let capture_index = m.captures[i - 1];
                    let captured_piece = self.squares[capture_index];
                    self.squares[capture_index] = '□';

                    // Update piece count
                    self.update_piece_count(captured_piece);
                }
            });
        } else {
            // Simple move or single capture
            self.squares[m.from] = '□';
            self.squares[m.to] = piece;

            // Process captures using iterators
            m.captures.iter().for_each(|&capture_index| {
                let captured_piece = self.squares[capture_index];
                self.squares[capture_index] = '□';

                // Update piece count
                self.update_piece_count(captured_piece);
            });
        }

        // Check if the piece should be promoted to a king
        if crate::mv::promote(self, m.to) {
            // Functional approach with pattern matching
            self.squares[m.to] = match self.squares[m.to] {
                'r' => 'R',
                'b' => 'B',
                other => other,
            };
        }

        // End Turn - use the toggle method
        self.turn = self.turn.toggle();

        Ok(())
    }

    // Helper function to update piece count
    fn update_piece_count(&mut self, captured_piece: char) {
        match captured_piece {
            'r' | 'R' => self.red_pieces -= 1,
            'b' | 'B' => self.black_pieces -= 1,
            _ => {},
        }
    }
}