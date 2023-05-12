// word_search.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 3
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
use csp;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

fn generate_grid(rows: usize, columns: usize) -> Grid {
    let mut grid = Vec::<Vec<char>>::new();
    let mut rng = thread_rng();
    for _ in 0..rows {
        let mut row_chars = Vec::<char>::new();
        for _ in 0..columns {
            row_chars.push(rng.gen_range('A'..'Z'));
        }
        grid.push(row_chars);
    }
    grid
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct GridLocation {
    row: usize,
    column: usize,
}

fn generate_domain(word: &str, grid: &Grid) -> Vec<Vec<GridLocation>> {
    let mut domain = Vec::<Vec<GridLocation>>::new();
    let height = grid.len();
    let width = grid[0].len();
    let length = word.len();
    for row in 0..height {
        for column in 0..width {
            if column + length <= width {
                domain.push(
                    (0..length)
                        .map(|o| GridLocation {
                            row: row,
                            column: column + o,
                        })
                        .collect::<Vec<GridLocation>>(),
                );
                if row + length <= height {
                    domain.push(
                        (0..length)
                            .map(|o| GridLocation {
                                row: row + o,
                                column: column + o,
                            })
                            .collect::<Vec<GridLocation>>(),
                    );
                }
            }
            if row + length <= height {
                domain.push(
                    (0..length)
                        .map(|o| GridLocation {
                            row: row + o,
                            column: column,
                        })
                        .collect::<Vec<GridLocation>>(),
                );
                if column >= length {
                    domain.push(
                        (0..length)
                            .map(|o| GridLocation {
                                row: row + o,
                                column: column - o,
                            })
                            .collect::<Vec<GridLocation>>(),
                    );
                }
            }
        }
    }
    domain
}

fn display_grid(grid: &Grid) {
    for row in grid {
        for c in row {
            print!("{c}");
        }
        println!("");
    }
}

#[derive(Clone)]
struct WordSearchConstraint {
    words: Vec<String>,
}

impl WordSearchConstraint {
    fn new(words: Vec<String>) -> Self {
        WordSearchConstraint { words }
    }
}

impl csp::Constraint<String, Vec<GridLocation>> for WordSearchConstraint {
    fn satisfied(&self, assignment: &HashMap<String, Vec<GridLocation>>) -> bool {
        let all_locations: Vec<&GridLocation> = assignment.values().flatten().collect();
        let deduplicated_locations: HashSet<&GridLocation> =
            HashSet::from_iter(all_locations.iter().cloned());
        all_locations.len() == deduplicated_locations.len()
    }
    fn variables(&self) -> Vec<String> {
        self.words.clone()
    }
}

fn main() {
    let words: Vec<String> = [
        "MATTHEW".to_string(),
        "JOE".to_string(),
        "MARY".to_string(),
        "SARAH".to_string(),
        "SALLY".to_string(),
    ]
    .to_vec();
    let mut locations = HashMap::<String, Vec<Vec<GridLocation>>>::new();
    let mut grid: Grid = generate_grid(9, 9);

    for word in &words {
        locations.insert(word.clone(), generate_domain(&word, &grid));
    }

    let mut csp =
        csp::CSP::<String, Vec<GridLocation>, WordSearchConstraint>::new(words.clone(), locations);
    csp.add_constraint(WordSearchConstraint::new(words));
    let solution = csp.backtracking_search();
    match solution {
        None => println!("No solution found!"),
        Some(solution) => {
            let mut rng = thread_rng();
            for (word, grid_locations) in solution.iter() {
                let mut locs = grid_locations.clone();
                if rng.gen_bool(0.5) {
                    locs.reverse();
                }
                for (index, letter) in word.chars().enumerate() {
                    let (row, column) = (locs[index].row, locs[index].column);
                    grid[row][column] = letter;
                }
            }
            display_grid(&grid);
        }
    }
}
