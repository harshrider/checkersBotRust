mod board;
mod mv;

use std::io::{self, Write};
use board::{Board, Color};

fn main() {
    println!("American Checkers");
    println!("Red (r/R) vs Black (b/B) □ Are real squares and ■ are not");
    println!("How to enter moves:");
    println!("1. Regular move:    'E3-F4'    (move one square diagonally)");
    println!("2. Capture move:    'B3-D5'    (jump over an opponent's piece)");
    println!("3. Multi-capture:   'B3-D5-B7' (multiple jumps in one turn)");
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
            break;
        }

        // Parse the move
        let move_result = mv::Move::from_notation(input, &board);
        match move_result {
            Some(m) => {
                // Check if the move is valid
                if mv::is_valid_move(&board, &m) {
                    println!("Executing move: {} with {} captures",
                             m.to_notation(&board),
                             m.captures.len());

                    // Execute the move
                    match board.make_move(&m) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Error executing move: {}", e);
                            continue;
                        }
                    }
                } else {
                    println!("Invalid move! Possible reasons:");
                    println!("- Not your piece");
                    println!("- Destination not empty");
                    println!("- Not moving diagonally");
                    println!("- Pawn moving in wrong direction");
                    println!("- Invalid capture");
                }
            },
            None => {
                println!("Invalid notation! Please use format like 'B3-D5'");
                println!("For captures, use the destination after the jump, not the captured piece location");
            }
        }
    }
}