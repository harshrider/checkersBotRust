// move.rs
// Handles move validation and generation for checkers

use crate::board::{Board, Color};

// Represents a move in the game
#[derive(Debug, Clone)]
pub struct Move {
    pub from: usize,  // Starting position (0-31)
    pub to: usize,    // Ending position (0-31)
    pub captures: Vec<usize>, // Indices of captured pieces
}
