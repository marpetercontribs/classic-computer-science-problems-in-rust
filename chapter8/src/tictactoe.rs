// tictactoe.rs
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

#[derive(Clone, PartialEq)]
pub enum TTTPiece {
    X,
    O,
    E,
}

impl ToString for TTTPiece {
    fn to_string(&self) -> String {
        match self {
            TTTPiece::X => "X".to_string(),
            TTTPiece::O => "O".to_string(),
            TTTPiece::E => " ".to_string(),
        }
    }
}
impl Piece for TTTPiece {
    fn opposite(&self) -> TTTPiece {
        match self {
            TTTPiece::X => TTTPiece::O,
            TTTPiece::O => TTTPiece::X,
            TTTPiece::E => TTTPiece::E,
        }
    }
}

pub struct TTTBoard {
    positions: Vec<TTTPiece>,
    turn: TTTPiece,
}

impl TTTBoard {
    pub fn new() -> Self {
        TTTBoard {
            positions: vec![TTTPiece::E; 9],
            turn: TTTPiece::X,
        }
    }
    pub fn new_from(positions: Vec<TTTPiece>, turn: TTTPiece) -> Self {
        TTTBoard { positions, turn }
    }
    fn check_position(&self, p0: usize, p1: usize, p2: usize) -> bool {
        self.positions[p0] == self.positions[p1]
            && self.positions[p1] == self.positions[p2]
            && self.positions[p0] != TTTPiece::E
    }
}

impl Board<TTTPiece, usize> for TTTBoard {
    fn turn(&self) -> TTTPiece {
        self.turn.clone()
    }
    fn do_move(&self, location: usize) -> TTTBoard {
        let mut positions = self.positions.clone();
        positions[location] = self.turn.clone();
        TTTBoard {
            positions,
            turn: self.turn.opposite(),
        }
    }
    fn legal_moves(&self) -> Vec<usize> {
        self.positions
            .iter()
            .enumerate()
            .filter_map(|(n, p)| if *p == TTTPiece::E { Some(n) } else { None })
            .collect()
    }
    fn is_win(&self) -> bool {
        self.check_position(0, 1, 2)
            || self.check_position(3, 4, 5)
            || self.check_position(6, 7, 8)
            || self.check_position(0, 3, 6)
            || self.check_position(1, 4, 7)
            || self.check_position(2, 5, 8)
            || self.check_position(0, 4, 8)
            || self.check_position(2, 4, 6)
    }
    fn evaluate(&self, player: &TTTPiece) -> f64 {
        if self.is_win() && self.turn == player.clone() {
            -1.0
        } else if self.is_win() {
            1.0
        } else {
            0.0
        }
    }
}

impl ToString for TTTBoard {
    fn to_string(&self) -> String {
        format!(
            "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}",
            self.positions[0].to_string(),
            self.positions[1].to_string(),
            self.positions[2].to_string(),
            self.positions[3].to_string(),
            self.positions[4].to_string(),
            self.positions[5].to_string(),
            self.positions[6].to_string(),
            self.positions[7].to_string(),
            self.positions[8].to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::minimax::find_best_move;

    #[test]
    fn test_easy_position() {
        let to_win_easy_position = vec![
            TTTPiece::X,
            TTTPiece::O,
            TTTPiece::X,
            TTTPiece::X,
            TTTPiece::E,
            TTTPiece::O,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::O,
        ];
        let board = TTTBoard::new_from(to_win_easy_position, TTTPiece::X);
        assert_eq!(find_best_move(&board, 8), Some(6));
    }

    #[test]
    fn test_block_position() {
        let to_block_position = vec![
            TTTPiece::X,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::O,
            TTTPiece::E,
            TTTPiece::X,
            TTTPiece::O,
        ];
        let board = TTTBoard::new_from(to_block_position, TTTPiece::X);
        assert_eq!(find_best_move(&board, 8), Some(2));
    }

    #[test]
    fn test_hard_position() {
        let to_win_hard_position = vec![
            TTTPiece::X,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::E,
            TTTPiece::O,
            TTTPiece::O,
            TTTPiece::X,
            TTTPiece::E,
        ];
        let board = TTTBoard::new_from(to_win_hard_position, TTTPiece::X);
        assert_eq!(find_best_move(&board, 8), Some(1));
    }
}
