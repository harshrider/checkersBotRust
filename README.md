# American Checkers AI

An implementation of American Checkers (Draughts) in Rust with Alpha-Beta and parallel AI opponents.

## Overview

This project implements a complete American Checkers game with two AI algorithms:
- Traditional Alpha-Beta pruning
- Parallel Loss Function (PLF) using Rayon

The focus is on comparing sequential vs. parallel approaches for game-tree search algorithms.

## Board Representation

The game uses a compact 32-element representation for the checkers board:

```rust
pub struct Board {
    pub squares: [char; 32],
    pub turn: Color,
    pub red_pieces: u8,
    pub black_pieces: u8,
}
```

Only the 32 dark squares are used for play, which simplifies move generation and reduces memory usage. Conversion functions map between this internal representation and standard 8×8 coordinates.

## Game Rules

This implementation follows American Checkers rules:
- Kings can move in any diagonal direction
- Captures are mandatory
- Multi-jump sequences are required when available
- Pieces promote to kings at the opponent's back rank

One limitation is that the move validator doesn't force players to capture when possible, though the AI algorithms correctly prioritize captures.

## Position Evaluation

The `bar` function evaluates board positions with the following considerations:

| Piece/Factor | Value/Effect |
|--------------|--------------|
| Red pawn     | +1.0         |
| Red king     | +2.5         |
| Black pawn   | -1.0         |
| Black king   | -2.5         |
| Edge squares | -0.25 penalty |
| Advancing pawns | Bonus based on proximity to promotion |
| Centralized kings | Bonus for controlling central squares |

Positive scores indicate advantage for Red, negative for Black.

## Alpha-Beta Algorithm

The classic minimax algorithm with alpha-beta pruning:

```rust
fn minimax_ab(board: &Board, depth: u32, is_maximizing_player: bool, 
              alpha: f32, beta: f32) -> (f32, Option<Move>)
```

This implementation uses Rust's functional features like `try_fold` for efficient pruning and pattern matching for readable code.

## Parallel Loss Function

The Parallel Loss Function distributes the search across multiple CPU cores:

```rust
pub fn parallel_loss_function(board: &Board, depth: u32) -> Option<Move>
```

The algorithm:
- Falls back to alpha-beta for shallow depths (≤5)
- Uses parallel evaluation at leaf nodes
- Evaluates all possible moves in parallel using Rayon

## Performance Comparison

| Depth | Alpha-Beta | Parallel | Better Algorithm | Avg AB Speed | Avg Par Speed |
|-------|------------|----------|------------------|--------------|---------------|
| 1-5   | Fast       | Slower   | Alpha-Beta       | 100-200ms    | 300-500ns     |
| 6-11  | Slow       | Faster   | Parallel         | 800ms-2s     | 500ms-1s      |
| 12+   | Very Slow  | Slower   | Alpha-Beta       | 10-13min     | 15min+        |

Parallelization is most beneficial at medium depths (6-11). At higher depths, the overhead of thread management and board cloning outweighs the benefits.

## Work and Span Analysis

- **Alpha-Beta**:
  - Work: O(b^d) in worst case
  - Span: Same as work (sequential)

- **Parallel Loss Function**:
  - Work: O(b × work-of-subtrees)
  - Span: O(d + b) theoretically

## Limitations and Future Work

- The bar function could be enhanced with more sophisticated evaluation criteria
- Principal Variation Search (PVS) could improve parallel performance
- Dynamic depth adjustment based on position complexity
- More efficient board cloning to reduce parallel overhead

## Conclusion

The project demonstrates that parallelization offers significant advantages within specific depth ranges but isn't universally beneficial. Understanding the problem characteristics and overhead involved is crucial for effective parallel algorithm design.
