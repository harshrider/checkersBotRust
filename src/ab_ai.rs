use crate::board::{Board, Color};
use crate::mv::Move;

// Eval bar
pub fn bar(board: &Board) -> f32 { // TODO: Simple eval bar with simple weights
    let mut bar: f32 = 0.0;

    for i in 0..32 {
        let piece = board.squares[i];
        if piece == '□' {
            continue;
        }

        let (row, _) = board.index_to_coords(i);

        let row_weight = match piece {
            'b' | 'R' => (row as f32 - 0.0) / 7.0,
            'r' | 'B' => (7.0 - row as f32) / 7.0,
            _ => 0.0
        };

        match piece {
            'r' => bar += 1.0 * row_weight  // Add positional value
            ,'R' =>bar += 3.0 * row_weight
            ,'b' => bar -= 1.0 * row_weight
            ,'B' => bar -= 3.0 * row_weight
            ,_ => {}
        }
    }

    bar
}
