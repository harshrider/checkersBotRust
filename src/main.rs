mod board;

use std::io::{self, Write};
use board::{Board, Color};

fn main() {
    println!("American Checkers");
    println!("Red (r/R) vs Black (b/B) □ Are real squares and ■ are not");

    let board = Board::new();
    board.display();



}