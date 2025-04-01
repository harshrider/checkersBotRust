mod board;
mod mv;

use std::io::{self, Write};
use board::{Board, Color};

fn process_move(board: &mut Board, input: &str) -> bool {
    // Quit command
    if input.to_lowercase() == "q" {
        println!("Thanks for playing!");
        return false;
    }

    // Parse the move
    let move_result = mv::Move::from_notation(input, board);
    match move_result {
        Some(m) => {
            // Check if the move is valid
            if mv::is_valid_move(board, &m) {
                println!("Executing move: {} with {} captures",
                         m.to_notation(board),
                         m.captures.len());

                // Execute the move
                match board.make_move(&m) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Error executing move: {}", e);
                    }
                }
            } else {
                println!("Invalid move! Possible reasons:");
                println!("- Not your piece");
                println!("- Destination not empty");
                println!("- Not moving diagonally");
                println!("- Pawn moving in wrong direction");
                println!("- Invalid capture");
                return true; // Continue the game even if move is invalid
            }
        },
        None => {
            println!("Invalid notation! Please use format like 'B3-D5'");
            println!("For captures, use the destination after the jump, not the captured piece location");
            return true; // Continue the game even if notation is invalid
        }
    }

    // Check if game is over
    if board.is_game_over() {
        if let Some(winner) = board.get_winner() {
            println!("{:?} wins!", winner);
        } else {
            println!("Draw!");
        }
        return false;
    }

    true
}

fn main() {
    println!("American Checkers");
    println!("Red (r/R) vs Black (b/B) □ Are real squares and ■ are not");
    println!("How to enter moves:");
    println!("1. Regular move:    'E3-F4'    (move one square diagonally)");
    println!("2. Capture move:    'B3-D5'    (jump over an opponent's piece)");
    println!("3. Multi-capture:   'B3-D5-B7' (multiple jumps in one turn)");
    println!("Type 'q' to quit the game");

    // Set this to true to run simulation, false for interactive mode
    let simulation_mode = true;

    // Predetermined moves for simulation
    let simulation_moves = [// TODO:This simulaited moves still is not correct as Captures Must Be Prioritzed 
        "E6-F5",
        "H3-G4",
        "F7-E6",
        "D3-C4",
        "E6-D5",
        "B3-A4",
        "A6-B5",
        "F3-E4",
        "G8-F7",
        "G4-E6-G8",
        "D5-B3",
        "E2-D3",
        "B5-C4",
        "D1-E2",
        "B3-D1",
        "G8-F7",
        "D1-F3-D5",
        "F7-H5"
    ];

    let mut board = Board::new();

    if simulation_mode {
        println!("\nRunning simulation with {} predetermined moves...", simulation_moves.len());

        for (i, &mv) in simulation_moves.iter().enumerate() {
            println!("\n--- Move {} ({:?}'s turn): {} ---", i + 1, board.turn, mv);
            board.display();

            if !process_move(&mut board, mv) {
                println!("Simulation ended early.");
                break;
            }


        }

        println!("\nFinal board state after simulation:");
        board.display();
    } else {
        // Interactive game loop
        loop {
            // Display the current board state
            board.display();

            println!("{:?}'s turn", board.turn);

            // Get the player's move
            print!("Enter your move: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if !process_move(&mut board, input) {
                break;
            }
        }
    }
}