// word_search_ex_alt.rs
// (More complicated) solution to exercise 1
// in Classic Computer Science Problems in Python/Java Chapter 3
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
use rand::Rng;
use std::collections::HashMap;
use std::rc::Rc;

type Grid = Vec<Vec<char>>;

fn generate_grid(rows: usize, columns: usize) -> Grid {
    let mut grid = Vec::<Vec<char>>::new();
    let mut rng = rand::rng();
    for _ in 0..rows {
        let mut row_chars = Vec::<char>::new();
        for _ in 0..columns {
            row_chars.push(rng.random_range('.'..='.'));
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
                            row,
                            column: column + o,
                        })
                        .collect::<Vec<GridLocation>>(),
                );
                domain.push(
                    (1..=length)
                        .map(|o| GridLocation {
                            row,
                            column: column + length - o,
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
                    domain.push(
                        (1..=length)
                            .map(|o| GridLocation {
                                row: row + length - o,
                                column: column + length - o,
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
                            column,
                        })
                        .collect::<Vec<GridLocation>>(),
                );
                domain.push(
                    (1..=length)
                        .map(|o| GridLocation {
                            row: row + length - o,
                            column,
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
                    domain.push(
                        (1..=length)
                            .map(|o| GridLocation {
                                row: row + length - o,
                                column: column - length + o,
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
        println!();
    }
}

struct WordSearchConstraint {
    words: Vec<String>,
}

impl WordSearchConstraint {
    fn new(words: Vec<String>) -> Self {
        WordSearchConstraint { words }
    }
}

fn difference(first: usize, last: usize) -> i32 {
    (first as i32) - (last as i32)
}

impl csp::Constraint<String, Vec<GridLocation>> for WordSearchConstraint {
    fn satisfied(&self, assignment: &HashMap<Rc<String>, Vec<GridLocation>>) -> bool {
        // instead of considering "overlapping" words invalid,
        // we have to check if the overlapping letters are the same
        let mut reduced_assignment = assignment.clone();
        // for each word
        for (word, locations) in assignment.iter() {
            reduced_assignment.remove(word); // avoid checking overlap of a word with itself or another word twice
            let delta_col = difference(locations[1].column, locations[0].column);
            let delta_row = difference(locations[1].row, locations[0].row);
            // check if the locations overlaps with another (remaining) word's
            for (other_word, other_locations) in reduced_assignment.iter() {
                let delta_other_col =
                    difference(other_locations[1].column, other_locations[0].column);
                let delta_other_row = difference(other_locations[1].row, other_locations[0].row);
                let determinant = delta_col * delta_other_row - delta_row * delta_other_col;
                let row_dist = difference(other_locations[0].row, locations[0].row);
                let col_dist = difference(other_locations[0].column, locations[0].column);
                if determinant == 0 {
                    // the two words are parallel
                    let row_dist_last =
                        difference(other_locations[other_word.len() - 1].row, locations[0].row);
                    let col_dist_last = difference(
                        other_locations[other_word.len() - 1].column,
                        locations[0].column,
                    );
                    if delta_col == 0 {
                        // both words run vertically
                        if locations[0].column == other_locations[0].column {
                            // and in the same column
                            let other_start = delta_row * row_dist;
                            let other_end = delta_row * row_dist_last;
                            if (other_start >= 0 && other_start < (word.len() as i32))
                                || (other_end >= 0 && other_end < (word.len() as i32))
                            {
                                // the other word's first or last letter are in the first word
                                return false;
                            }
                        }
                    } else if delta_row == 0 {
                        // both words run horizontally
                        if locations[0].row == other_locations[0].row {
                            // and in the same row
                            let other_start = delta_col * col_dist;
                            let other_end = delta_col * col_dist_last;
                            if (other_start >= 0 && other_start < (word.len() as i32))
                                || (other_end >= 0 && other_end < (word.len() as i32))
                            {
                                // the other word's first or last letter are in the first word
                                return false;
                            }
                        }
                    } else {
                        // both words run diagonal - to be detailed!
                        let other_word_first_col = delta_col * col_dist;
                        let other_word_first_row = delta_row * row_dist;
                        let other_word_last_col = delta_col * col_dist_last;
                        let other_word_last_row = delta_row * row_dist_last;
                        if (other_word_first_col == other_word_first_row
                            && other_word_first_col >= 0
                            && other_word_first_col < (word.len() as i32))
                            || (other_word_last_col == other_word_last_row
                                && other_word_last_col >= 0
                                && other_word_last_col < (word.len() as i32))
                        {
                            return false;
                        }
                    }
                } else {
                    let other_word_index =
                        (delta_row * col_dist - delta_col * row_dist) * determinant;
                    if other_word_index >= 0 && other_word_index < (other_word.len() as i32) {
                        let word_index =
                            (delta_other_row * col_dist - delta_other_col * row_dist) * determinant;
                        if word_index >= 0
                            && word_index < (word.len() as i32)
                            && word.char_indices().nth(word_index as usize)
                                != other_word.char_indices().nth(other_word_index as usize)
                        {
                            return false;
                        };
                    }
                }
            }
        }
        true
    }
    fn variables(&self) -> &Vec<String> {
        &self.words
    }
}

fn main() {
    let words: Vec<String> = [
        "MATTHEW".to_string(),
        "JOE".to_string(),
        "MARY".to_string(),
        "SARAH".to_string(),
        "SALLY".to_string(),
        "JONATHAN".to_string(),
        "DANIEL".to_string(),
        "ISAAC".to_string(),
    ]
    .to_vec();
    let mut locations = HashMap::<String, Vec<Vec<GridLocation>>>::new();
    let mut grid: Grid = generate_grid(9, 9);

    for word in &words {
        locations.insert(word.clone(), generate_domain(word, &grid));
    }

    let mut csp = csp::CSP::<String, Vec<GridLocation>, WordSearchConstraint>::new(locations);
    csp.add_constraint(WordSearchConstraint::new(words));
    let solution = csp.backtracking_search();
    match solution {
        None => println!("No solution found!"),
        Some(solution) => {
            for (word, grid_locations) in solution.iter() {
                for (index, letter) in word.chars().enumerate() {
                    let (row, column) = (grid_locations[index].row, grid_locations[index].column);
                    grid[row][column] = letter;
                }
            }
            display_grid(&grid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csp::Constraint;

    #[test]
    fn test_cross_on_first_letters() {
        let words: Vec<String> = ["MATTHEW".to_string(), "MARY".to_string()].to_vec();
        let mut assignment_1 = HashMap::<Rc<String>, Vec<GridLocation>>::new();
        assignment_1.insert(
            Rc::new(words[0].clone()),
            (0..words[0].len())
                .map(|column| GridLocation { row: 0, column })
                .collect::<Vec<GridLocation>>(),
        );
        assignment_1.insert(
            Rc::new(words[1].clone()),
            (0..words[1].len())
                .map(|row| GridLocation { row, column: 0 })
                .collect::<Vec<GridLocation>>(),
        );
        let constraint = WordSearchConstraint::new(words.clone());
        assert_eq!(constraint.satisfied(&assignment_1), true);
    }

    #[test]
    fn test_cross_on_first_with_second_letter() {
        let words: Vec<String> = ["MATTHEW".to_string(), "MARY".to_string()].to_vec();
        let mut assignment_1 = HashMap::<Rc<String>, Vec<GridLocation>>::new();
        assignment_1.insert(
            Rc::new(words[0].clone()),
            (0..words[0].len())
                .map(|column| GridLocation { row: 0, column })
                .collect::<Vec<GridLocation>>(),
        );
        assignment_1.insert(
            Rc::new(words[1].clone()),
            (0..words[1].len())
                .map(|row| GridLocation { row, column: 1 })
                .collect::<Vec<GridLocation>>(),
        );
        let constraint = WordSearchConstraint::new(words.clone());
        assert_eq!(constraint.satisfied(&assignment_1), false);
    }
}
