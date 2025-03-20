#[derive(Debug, Clone)]
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

        // Set up black pieces (top of board)
        for i in 0..11 {
            squares[i] = 'b';
        }

        // Set up red pieces (bottom of board)
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

    // Convert between 2D coordinates (row, col) and 1D index (0-31)
    pub fn coords_to_index(&self, row: usize, col: usize) -> Option<usize> {
        // Check if the coordinates are valid for a black square
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
            //println!("\t_____\t_____\t_____\t_____\t_____\t_____\t_____\t_____");
        }

        println!("Turn: {:?}", self.turn);
        println!("Red pieces: {}, Black pieces: {}", self.red_pieces, self.black_pieces);
    }
}