use crate::mv::Move;

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

impl Board {
    // Create a new board with the initial setup
    pub fn new() -> Self {
        let mut squares = ['□'; 32];

        // black pieces
        for i in 0..12 {
            squares[i] = 'b';
        }

        // red pieces
        for i in 20..32 {
            squares[i] = 'r';
        }

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
        // Bounds check
        if row >= 8 || col >= 8 {
            return None;
        }

        // Only dark squares are valid in checkers
        if (row + col) % 2 == 0 {
            return None; // Light squares are not used
        }

        // Calculate index in the 1D array
        // Each row has 4 valid squares
        // Even rows have pieces at odd columns (B0, D0, F0, H0)
        // Odd rows have pieces at even columns (A1, C1, E1, G1)
        let index = if row % 2 == 0 {
            // Even row (0, 2, 4, 6) - valid squares are at odd columns
            (row / 2) * 8 + (col / 2)
        } else {
            // Odd row (1, 3, 5, 7) - valid squares are at even columns
            (row / 2) * 8 + 4 + (col / 2)
        };

        if index < 32 {
            Some(index)
        } else {
            None
        }
    }

    pub fn display(&self) {
        println!("\tA\tB\tC\tD\tE\tF\tG\tH");
        println!("\t_____\t_____\t_____\t_____\t_____\t_____\t_____\t_____");

        for row in 0..8 {
            print!("{}|\t", row+1);

            for col in 0..8 {
                let piece = if (row + col) % 2 == 1 {
                    if let Some(index) = self.coords_to_index(row, col) {
                        self.squares[index]
                    } else {
                        ' '
                    }
                } else {
                    '■'
                };

                print!("|{}|\t", piece);
            }

            println!();
        }

        println!("Turn: {:?}", self.turn);
        println!("Red pieces: {}, Black pieces: {}", self.red_pieces, self.black_pieces);
    }

    // Check if the game is over - for now, only when all pieces are captured
    pub fn is_game_over(&self) -> bool {
        self.red_pieces == 0 || self.black_pieces == 0
    }

    // Calculate Winner - simplified version
    pub fn get_winner(&self) -> Option<Color> {
        if self.red_pieces == 0 {
            Some(Color::Black)
        } else if self.black_pieces == 0 {
            Some(Color::Red)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_valid_moves(&self) -> Vec<Move> {
        // This will be implemented in a separate file later
        Vec::new()
    }

    // Execute a move on the board
    pub fn make_move(&mut self, m: &crate::mv::Move) -> Result<(), &'static str> {
        // Move the piece from source to destination
        let piece = self.squares[m.from];
        self.squares[m.from] = '□';
        self.squares[m.to] = piece;

        for &capture_index in &m.captures {
            let captured_piece = self.squares[capture_index];
            self.squares[capture_index] = '□';

            // Update piece count
            if captured_piece == 'r' || captured_piece == 'R' {
                self.red_pieces -= 1;
            } else if captured_piece == 'b' || captured_piece == 'B' {
                self.black_pieces -= 1;
            }
        }

        // End Turn
        self.turn = match self.turn {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };

        Ok(())
    }
}