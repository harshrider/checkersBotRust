mod board;
mod mv;

use std::io::{self, Write};
use board::{Board, Color};

fn main() {
    println!("American Checkers");
    println!("Red (r/R) vs Black (b/B) □ Are real squares and ■ are not");

    let board = Board::new();

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

        //Quit
        if input.to_lowercase() == "q" {break;}



        // Parse the move
        if let Some(m) = mv::Move::from_notation(input, &board) {
            // Check if the move is valid
            if mv::is_valid_move(&board, &m) {
                // Execute the move
                match board.make_move(&m) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    }
                }
            } else {
                println!("Invalid move! Please try again.");
            }
        } else {
            println!("Invalid notation! Please use algebraic notation (e.g., 'E3-F4' or 'E3xG5')");
        }
    }
}