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
        for i in 0..11 {
            squares[i] = 'b';
        }

        // red pieces
        for i in 19..32 {
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
        let row = index / 4;
        let col = if row % 2 == 0 {
            index % 4 * 2 + 1
        } else {
            index % 4 * 2
        };
        (row, col)
    }


    pub fn coords_to_index(&self, row: usize, col: usize) -> Option<usize> {
        if (row + col) % 2 == 0 {
            return None; // White squares
        }

        let index = row * 4 + col / 2 - if row % 2 == 0 { 0 } else { 1 };

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
            print!("{}|\t", row);

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

    // Check if the game is over
    pub fn is_game_over(&self) -> bool {
        self.red_pieces == 0 || self.black_pieces == 0
    }

    // Calculate Winner
    pub fn get_winner(&self) -> Option<Color> { // Checks for all 3 cases
        if self.red_pieces == 0 {
            Some(Color::Black)
        } else if self.black_pieces == 0 {
            Some(Color::Red)
        } else if self.get_valid_moves().is_empty() {
            Some(match self.turn {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            })
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn get_valid_moves(&self) -> Vec<Move> {// Temp Function
        // todo
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

