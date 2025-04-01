mod board;
mod mv;

use std::io::{self, Write};
use board::{Board, Color};

fn main() {
    println!("American Checkers");
    println!("Red (r/R) vs Black (b/B) □ Are real squares and ■ are not");
    println!("Enter moves in algebraic notation (e.g., 'E3-F4' for regular move or 'E3xG5' for a capture)");
    println!("Type 'q' to quit the game");

    let mut board = Board::new();

    // Game loop
    loop {
        // Display the current board state
        board.display();

        // Check if the game is over
        if board.is_game_over() {
            if let Some(winner) = board.get_winner() {
                println!("{:?} wins!", winner);
            } else {
                println!("Draw!");
            }
            break;
        }

        println!("{:?}'s turn", board.turn);

        // Get the player's move
        print!("Enter your move: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Quit
        if input.to_lowercase() == "q" {
            println!("Thanks for playing!");
            break;
        }

        println!(" input: '{}'", input);

        // Parse the move
        let move_result = mv::Move::from_notation(input, &board);
        match move_result {
            Some(m) => {
                println!("DEBUG: parsed ");
                println!("From index: {}, To index: {}", m.from, m.to);
                println!(" From : {:?}, To : {:?}",
                         board.index_to_coords(m.from),
                         board.index_to_coords(m.to));
                println!("Piece at from: '{}'", board.squares[m.from]);
                println!("Piece at to: '{}'", board.squares[m.to]);
                println!("Captures: {:?}", m.captures);

                // Check if the move is valid
                if mv::is_valid_move(&board, &m) {

                    // Execute the move
                    match board.make_move(&m) {
                        Ok(_) => {
                        },
                        Err(e) => {
                            println!("Error executing move: {}", e);
                            continue;
                        }
                    }
                } else {


                    // Print move details for debugging
                    let (from_row, from_col) = board.index_to_coords(m.from);
                    let (to_row, to_col) = board.index_to_coords(m.to);
                    println!("From: ({},{}) To: ({},{})", from_row, from_col, to_row, to_col);
                    println!("Row diff: {}, Col diff: {}",
                             (from_row as i32 - to_row as i32).abs(),
                             (from_col as i32 - to_col as i32).abs());
                }
            },
            None => {
                println!("Invalid notation! Couldn't parse the move '{}'", input);
                println!("- Not your piece");
                println!("- Destination not empty");
                println!("- Not moving diagonally");
                println!("- Pawn moving in wrong direction");
                println!("- Invalid capture");

            }
        }
    }
}