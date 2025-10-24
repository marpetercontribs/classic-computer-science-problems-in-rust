// connectfour.rs
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
use once_cell::sync::Lazy;

#[derive(Clone, PartialEq)]
pub enum C4Piece {
    B,
    R,
    E,
}

impl Piece for C4Piece {
    fn opposite(&self) -> C4Piece {
        match self {
            C4Piece::B => C4Piece::R,
            C4Piece::R => C4Piece::B,
            C4Piece::E => C4Piece::E,
        }
    }
}

impl std::fmt::Display for C4Piece {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                C4Piece::B => "B",
                C4Piece::R => "R",
                C4Piece::E => " ",
            }
        )
    }
}

fn generate_segments(
    num_columns: usize,
    num_rows: usize,
    segment_length: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut segments: Vec<Vec<(usize, usize)>> = Vec::new();
    // generate the vertical segments
    for col in 0..num_columns {
        for row in 0..=(num_rows - segment_length) {
            let mut segment = Vec::<(usize, usize)>::with_capacity(segment_length);
            for i in 0..segment_length {
                segment.push((col, row + i));
            }
            segments.push(segment);
        }
    }
    // generate the horizontal segments
    for col in 0..=(num_columns - segment_length) {
        for row in 0..num_rows {
            let mut segment = Vec::<(usize, usize)>::with_capacity(segment_length);
            for i in 0..segment_length {
                segment.push((col + i, row));
            }
            segments.push(segment);
        }
    }
    // generate the bottom left to top right diagonal segments
    for col in 0..=(num_columns - segment_length) {
        for row in 0..=(num_rows - segment_length) {
            let mut segment = Vec::<(usize, usize)>::with_capacity(segment_length);
            for i in 0..segment_length {
                segment.push((col + i, row + i));
            }
            segments.push(segment);
        }
    }
    // generate the top left to bottom right diagonal segments
    for col in 0..=(num_columns - segment_length) {
        for row in (segment_length - 1)..num_rows {
            let mut segment = Vec::<(usize, usize)>::with_capacity(segment_length);
            for i in 0..segment_length {
                segment.push((col + i, row - i));
            }
            segments.push(segment);
        }
    }
    segments
}

const NUM_COLUMNS: usize = 7;
const NUM_ROWS: usize = 6;
const SEGMENT_LENGTH: usize = 4;
static SEGMENTS: Lazy<Vec<Vec<(usize, usize)>>> =
    Lazy::new(|| generate_segments(NUM_COLUMNS, NUM_ROWS, SEGMENT_LENGTH));

pub struct C4Board {
    positions: Vec<Vec<C4Piece>>,
    column_level: Vec<usize>,
    turn: C4Piece,
}

impl C4Board {
    pub fn new() -> Self {
        C4Board {
            positions: vec![vec![C4Piece::E; NUM_ROWS]; NUM_COLUMNS],
            column_level: vec![0; NUM_COLUMNS],
            turn: C4Piece::B,
        }
    }
    fn new_from(positions: Vec<Vec<C4Piece>>, turn: C4Piece) -> Self {
        let column_level =
            positions
                .iter()
                .fold(Vec::with_capacity(positions.len()), |mut vec, column| {
                    vec.push(column.iter().fold(0, |count, p| {
                        if *p == C4Piece::B || *p == C4Piece::R {
                            count + 1
                        } else {
                            count
                        }
                    }));
                    vec
                });
        C4Board {
            positions,
            column_level,
            turn,
        }
    }
    fn count_segment(&self, segment: &[(usize, usize)]) -> (usize, usize) {
        segment.iter().fold(
            (0_usize, 0_usize),
            |(black_count, red_count), (column, row)| {
                if self.positions[*column][*row] == C4Piece::B {
                    (black_count + 1, red_count)
                } else if self.positions[*column][*row] == C4Piece::R {
                    (black_count, red_count + 1)
                } else {
                    (black_count, red_count)
                }
            },
        )
    }
    fn evaluate_segment(&self, segment: &[(usize, usize)], player: &C4Piece) -> f64 {
        let (black_count, red_count) = self.count_segment(segment);
        if black_count > 0 && red_count > 0 {
            return 0_f64; // mixed segments are neutral
        }
        let count: usize = red_count.max(black_count);
        let mut score: f64 = 0_f64;
        if count == 2 {
            score = 1_f64;
        } else if count == 3 {
            score = 100_f64;
        } else if count == 4 {
            score = 1000000_f64;
        }
        let winner: C4Piece = if red_count > black_count {
            C4Piece::R
        } else {
            C4Piece::B
        };
        if winner == *player {
            return score;
        }
        -score
    }
}

impl Default for C4Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board<C4Piece, usize> for C4Board {
    fn turn(&self) -> C4Piece {
        self.turn.clone()
    }
    fn do_move(&self, location: usize) -> C4Board {
        let mut positions = self.positions.clone();
        positions[location][self.column_level[location]] = self.turn.clone();
        C4Board::new_from(positions, self.turn.opposite())
    }
    fn legal_moves(&self) -> Vec<usize> {
        self.column_level
            .iter()
            .enumerate()
            .filter_map(|(column, level)| {
                if level < &NUM_ROWS {
                    Some(column)
                } else {
                    None
                }
            })
            .collect()
    }
    fn is_win(&self) -> bool {
        for segment in SEGMENTS.iter() {
            let (black_count, red_count) = self.count_segment(segment);
            if black_count == 4 || red_count == 4 {
                return true;
            }
        }
        false
    }
    fn evaluate(&self, player: &C4Piece) -> f64 {
        SEGMENTS.iter().fold(0_f64, |total, segment| {
            total + self.evaluate_segment(segment, player)
        })
    }
}

impl std::fmt::Display for C4Board {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 1..=NUM_ROWS {
            write!(formatter, "|")?;
            for col in 0..NUM_COLUMNS {
                write!(formatter, "{}|", self.positions[col][NUM_ROWS - row])?;
            }
            writeln!(formatter)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::minimax::find_best_move;

    fn three_reds_three_blacks_board() -> Vec<Vec<C4Piece>> {
        let three_reds = vec![C4Piece::R;3];
        let three_blacks = vec![C4Piece::B;3];
        let mut column_one = three_reds.clone();
        column_one.extend(three_blacks.clone());
        let mut column_two = three_blacks;
        column_two.extend(three_reds);      

        vec![
            column_one.clone(), column_two.clone(),
            column_one.clone(), column_two.clone(),
            column_one.clone(), column_two.clone(),
            column_one, column_two]
    }

    fn incomplete_three_reds_three_blacks_board() -> Vec<Vec<C4Piece>> {
        let three_reds = vec![C4Piece::R;3];
        let three_blacks = vec![C4Piece::B;3];
        let mut column_one = three_reds.clone();
        column_one.extend(three_blacks.clone());
        let mut column_two = three_blacks;
        column_two.extend(three_reds);      

        vec![
            column_one.clone(), column_two.clone(),
            column_one.clone(), column_two.clone(),
            column_one.clone(), column_two.clone(),
            column_one, vec![C4Piece::E;3]]
    }

    fn half_three_reds_three_blacks_board() -> Vec<Vec<C4Piece>> {
        let mut column_one = vec![C4Piece::R;3];
        column_one.extend(vec![C4Piece::E;3]);
        let mut column_two = vec![C4Piece::B;3];
        column_two.extend(vec![C4Piece::E;3]);      

        vec![
            column_one.clone(), column_two.clone(),
            column_one.clone(), column_two.clone(),
            column_one.clone(), column_two.clone(),
            column_one, column_two]
    }

    #[test]
    fn test_is_draw_detected() {
        let draw_position = three_reds_three_blacks_board();
        let board = C4Board::new_from(draw_position, C4Piece::R);
        assert!(board.is_draw());
    }

    #[test]
    fn test_is_no_draw_detected() {
        let board = C4Board::new_from(incomplete_three_reds_three_blacks_board(), C4Piece::R);
        assert!(!board.is_draw());
        let board = C4Board::new_from(half_three_reds_three_blacks_board(), C4Piece::R);
        assert!(!board.is_draw());
    }

    #[test]
    fn test_is_vertical_win_detected() {
        let mut winning_board = half_three_reds_three_blacks_board();
        winning_board[2][3] = C4Piece::R;
        let board = C4Board::new_from(winning_board, C4Piece::R);
        assert!(board.is_win());
        let mut winning_board = half_three_reds_three_blacks_board();
        winning_board[1][3] = C4Piece::B;   
        let board = C4Board::new_from(winning_board, C4Piece::R);
        assert!(board.is_win());
    }

    #[test]
    fn test_is_horizontal_win_detected() {
        let mut winning_board = half_three_reds_three_blacks_board();
        winning_board[1][1] = C4Piece::R;
        winning_board[3][1] = C4Piece::R;
        let board = C4Board::new_from(winning_board, C4Piece::R);
        assert!(board.is_win());
    }

    #[test]
    fn test_is_diagonal_win_detected() {
        let mut winning_board = half_three_reds_three_blacks_board();
        winning_board[1][1] = C4Piece::R;
        winning_board[3][3] = C4Piece::R;
        let board = C4Board::new_from(winning_board, C4Piece::R);
        assert!(board.is_win());
        let mut winning_board = half_three_reds_three_blacks_board();
        winning_board[0][3] = C4Piece::B;
        winning_board[2][1] = C4Piece::B;
        let board = C4Board::new_from(winning_board, C4Piece::R);
        assert!(board.is_win());
    }

    #[test]
    fn test_is_no_win_detected_draw() {
        let draw_position = three_reds_three_blacks_board();
        let board = C4Board::new_from(draw_position, C4Piece::R);
        assert!(!board.is_win());
    }

    #[test]
    fn test_is_no_win_detected_open() {
        let board = C4Board::new_from(incomplete_three_reds_three_blacks_board(), C4Piece::R);
        assert!(!board.is_win());
        let board = C4Board::new_from(half_three_reds_three_blacks_board(), C4Piece::R);
        assert!(!board.is_win());
    }

    #[test]
    fn test_legal_moves_non_empty() {
        let board = C4Board::new_from(incomplete_three_reds_three_blacks_board(), C4Piece::R);
        assert_eq!(board.legal_moves(),vec![7]);
        let board = C4Board::new_from(half_three_reds_three_blacks_board(), C4Piece::R);
        assert_eq!(board.legal_moves(),vec![0, 1, 2, 3, 4, 5, 6, 7]);

    }

    #[test]
    fn test_legal_moves_empty() {
        let board = C4Board::new_from(three_reds_three_blacks_board(), C4Piece::R);
        assert_eq!(board.legal_moves(),vec![]);
        let board = C4Board::new_from(three_reds_three_blacks_board(), C4Piece::B);
        assert_eq!(board.legal_moves(),vec![]);
    }

    #[test]
    fn test_easy_position() {
        let mut position = vec![vec![C4Piece::E; NUM_ROWS]; NUM_COLUMNS];
        position[0][0] = C4Piece::B;
        position[1][1] = C4Piece::B;
        position[2][2] = C4Piece::B;
        position[1][0] = C4Piece::R;
        position[2][0] = C4Piece::R;
        position[2][1] = C4Piece::R;
        position[3][0] = C4Piece::R;
        position[3][1] = C4Piece::B;
        position[3][2] = C4Piece::R;
        let board = C4Board::new_from(position, C4Piece::R);
        println!("{}",board.to_string());
        assert_eq!(find_best_move(&board, 6), Some(4));
    }

    #[test]
    fn test_block_horizontal_position() {
        let mut position = vec![vec![C4Piece::E; NUM_ROWS]; NUM_COLUMNS];
        position[0][0] = C4Piece::B;
        position[1][0] = C4Piece::B;
        position[2][0] = C4Piece::B;
        position[4][0] = C4Piece::R;
        position[0][1] = C4Piece::R;
        let board = C4Board::new_from(position, C4Piece::R);
        assert_eq!(find_best_move(&board, 6), Some(3));
    }


    #[test]
    fn test_block_vertical_position() {
        let mut position = vec![vec![C4Piece::E; NUM_ROWS]; NUM_COLUMNS];
        position[1][0] = C4Piece::B;
        position[1][1] = C4Piece::B;
        position[1][2] = C4Piece::B;
        position[2][0] = C4Piece::R;
        position[3][0] = C4Piece::R;
        let board = C4Board::new_from(position, C4Piece::R);
        assert_eq!(find_best_move(&board, 6), Some(1));
    }

    #[test]
    fn test_block_diagonal_position() {
        let mut position = vec![vec![C4Piece::E; NUM_ROWS]; NUM_COLUMNS];
        position[0][0] = C4Piece::B;
        position[1][1] = C4Piece::B;
        position[2][2] = C4Piece::B;
        position[1][0] = C4Piece::R;
        position[2][0] = C4Piece::R;
        position[2][1] = C4Piece::R;
        position[3][0] = C4Piece::R;
        position[3][1] = C4Piece::B;
        position[3][2] = C4Piece::R;
        position[4][0] = C4Piece::B;
        let board = C4Board::new_from(position, C4Piece::R);
        println!("{}",board.to_string());
        assert_eq!(find_best_move(&board, 6), Some(3));
    }
}