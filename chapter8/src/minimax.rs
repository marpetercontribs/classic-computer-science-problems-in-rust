// minimax.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 8
// Copyright 2023 Markus Peter
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::board::{Board, Piece};

// Find the best possible outcome for original player
pub fn minimax<P: Piece, Move: Copy, B: Board<P, Move> + Sized>(
    board: &B,
    maximizing: bool,
    original_player: &P,
    max_depth: usize,
) -> f64 {
    // Base case – terminal position or maximum depth reached
    if board.is_win() || board.is_draw() || max_depth == 0 {
        return board.evaluate(original_player);
    }
    // Recursive case - maximize your gains or minimize the opponent's gains
    if maximizing {
        let mut best_eval = f64::NEG_INFINITY;
        for a_move in board.legal_moves().iter() {
            let result = minimax(
                &board.do_move(*a_move),
                false,
                original_player,
                max_depth - 1,
            );
            best_eval = f64::max(best_eval, result);
        }
        best_eval
    } else {
        let mut worst_eval = f64::INFINITY;
        for a_move in board.legal_moves().iter() {
            let result = minimax(
                &board.do_move(*a_move),
                true,
                original_player,
                max_depth - 1,
            );
            worst_eval = f64::min(worst_eval, result);
        }
       worst_eval
    }
}

// Find the best possible move in the current position
// looking up to max_depth ahead
pub fn find_best_move<P: Piece, Move: Copy, B: Board<P, Move> + Sized>(
    board: &B,
    max_depth: usize,
) -> Option<Move> {
    let mut best_eval = f64::NEG_INFINITY;
    let mut best_move: Option<Move> = None; // Declaration as Move is not sufficient because the following loop may not initialize best_move
    for a_move in board.legal_moves().iter() {
        let result = minimax(&board.do_move(*a_move), false, &board.turn(), max_depth - 1);
        if result > best_eval {
            best_eval = result;
            best_move = Some(*a_move);
        }
    }
    best_move
}
