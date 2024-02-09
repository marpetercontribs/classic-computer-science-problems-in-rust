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

impl ToString for C4Piece {
    fn to_string(&self) -> String {
        match self {
            C4Piece::B => "B".to_string(),
            C4Piece::R => "R".to_string(),
            C4Piece::E => " ".to_string(),
        }
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

impl ToString for C4Board {
    fn to_string(&self) -> String {
        let mut result: String = String::new();
        for row in 1..=NUM_ROWS {
            result.push('|');
            for col in 0..NUM_COLUMNS {
                result.push_str(&self.positions[col][NUM_ROWS - row].to_string());
                result.push('|');
            }
            result.push('\n');
        }
        result
    }
}
