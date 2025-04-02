use std::collections::HashMap;
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
            ,'R' =>bar += 2.0+ 1.5 * row_weight
            ,'b' => bar -= 1.0 * row_weight
            ,'B' => bar -= 2.0+ 1.5 * row_weight
            ,_ => {}
        }
    }

    (bar * 100.0).round() / 100.0
}


pub fn ab_ai(board: &Board, depth: u32) -> Option<Move> {
    let valid_moves = board.get_valid_moves();

    if valid_moves.is_empty() {
        return None;
    }

    let mut move_values: HashMap<String, f32> = HashMap::new();

    let is_maximizing = board.turn == Color::Red;

    let (value, best_move) = minimax_ab(board, depth, is_maximizing, f32::NEG_INFINITY, f32::INFINITY);

    let best_move = best_move.unwrap_or_else(|| valid_moves[0].clone());

    println!("Best move: {} with value: {}", best_move.to_notation(board), value);

    Some(best_move)
}

fn minimax_ab(board: &Board, depth: u32, is_maximizing_player: bool, mut alpha: f32, mut beta: f32) -> (f32, Option<Move>) {
    if depth == 0 || board.is_game_over() {
        if board.is_game_over() {
            if let Some(winner) = board.get_winner() {
                return match winner {
                    Color::Red => (f32::INFINITY, None),       // Red wins
                    Color::Black => (f32::NEG_INFINITY, None), // Black wins
                };
            }
            return (0.0, None); // Draw
        }

        return (bar(board), None);
    }

    let valid_moves = board.get_valid_moves();

    if valid_moves.is_empty() {
        return if is_maximizing_player {
            (f32::NEG_INFINITY, None)
        } else {
            (f32::INFINITY, None)
        };
    }
    if valid_moves.len() == 1 {
        return (bar(board), valid_moves.first().cloned());
    }

    if is_maximizing_player {
        valid_moves.into_iter()
            .try_fold((f32::NEG_INFINITY, None, alpha), |(best_val, best_move, alpha), mv| {
                if beta <= alpha {
                    return Err((best_val, best_move));
                }

                let mut new_board = board.clone();
                let _ = new_board.make_move(&mv);

                let (value, _) = minimax_ab(&new_board, depth - 1, false, alpha, beta);

                // Update best value, move, and alpha
                if value > best_val {
                    let new_alpha = alpha.max(value);
                    Ok((value, Some(mv.clone()), new_alpha))
                } else {
                    Ok((best_val, best_move, alpha))
                }
            })
            .map_or_else(
                |early_result| early_result,
                |(final_best, final_move, _)| (final_best, final_move)
            )
    } else {
        valid_moves.into_iter()
            .try_fold((f32::INFINITY, None, beta), |(best_val, best_move, beta), mv| {
                if beta <= alpha {
                    return Err((best_val, best_move));
                }

                let mut new_board = board.clone();
                let _ = new_board.make_move(&mv);

                let (value, _) = minimax_ab(&new_board, depth - 1, true, alpha, beta);

                if value < best_val {
                    let new_beta = beta.min(value);
                    Ok((value, Some(mv.clone()), new_beta))
                } else {
                    Ok((best_val, best_move, beta))
                }
            })
            .map_or_else(
                |early_result| early_result,
                |(final_best, final_move, _)| (final_best, final_move)
            )
    }
}