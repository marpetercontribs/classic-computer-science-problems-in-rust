// sudoku.rs
// Solution to exercise 3
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

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::rc::Rc;

#[derive(Hash, PartialEq, Eq, Clone)]
struct CellLocation {
    row: u8,
    column: u8,
}

#[derive(Clone)]
struct Puzzle {
    grid: Vec<Vec<u8>>,
}

impl Puzzle {
    // fn new(rows: usize, columns: usize) -> Self {
    //     let mut grid = Vec::<Vec<usize>>::new();
    //     for _ in 0..rows {
    //         let mut row = Vec::<usize>::new();
    //         for _ in 0..columns {
    //             row.push(0);
    //         }
    //         grid.push(row);
    //     }
    //     Puzzle { grid }
    // }
    fn from_file(file_name: &String) -> Self {
        let file = File::open(file_name).expect("Cannot read file");
        let lines = io::BufReader::new(file).lines();
        let mut grid = Vec::<Vec<u8>>::with_capacity(9);
        for line in lines {
            let mut row = Vec::<u8>::with_capacity(9);
            for cell in line.unwrap().chars() {
                if cell == ' ' {
                    row.push(0);
                } else {
                    row.push(cell.to_digit(10).unwrap() as u8);
                }
            }
            grid.push(row);
        }
        Puzzle { grid }
    }
    fn display(&self) {
        for row in &self.grid {
            for value in row {
                print!("{value}");
            }
            println!();
        }
    }
    fn update(&mut self, assignment: &HashMap<Rc<CellLocation>, u8>) {
        for (cell_location, value) in assignment.iter() {
            self.grid[cell_location.row as usize][cell_location.column as usize] = *value;
        }
    }
}

struct SudokuConstraint {
    locations: Vec<CellLocation>,
}

impl SudokuConstraint {
    fn new(locations: Vec<CellLocation>) -> Self {
        SudokuConstraint { locations }
    }
}

impl csp::Constraint<CellLocation, u8> for SudokuConstraint {
    fn satisfied(&self, assignment: &HashMap<Rc<CellLocation>, u8>) -> bool {
        for cell_location in assignment.keys() {
            let value = assignment.get(cell_location);
            // only other locations with the same value assigned have to be checked
            for checked_location in assignment
                .keys()
                .filter(|loc| &cell_location != loc && value == assignment.get(*loc))
            {
                // check if the other cell is in the same row or column
                if cell_location.row == checked_location.row
                    || cell_location.column == checked_location.column
                {
                    return false;
                }
                // check if the other cell is in the same sub-grid
                let subgrid_row: u8 = cell_location.row / 3;
                let subgrid_column: u8 = cell_location.column / 3;
                let checked_subgrid_column: u8 = checked_location.column / 3;
                let checked_subgrid_row: u8 = checked_location.row / 3;
                if subgrid_row == checked_subgrid_row && subgrid_column == checked_subgrid_column {
                    return false;
                }
            }
        }
        true
    }
    fn variables(&self) -> &Vec<CellLocation> {
        &self.locations
    }
}

fn main() {
    let mut puzzle = Puzzle::from_file(&String::from("easy_puzzle.txt"));
    // let mut puzzle = Puzzle::from_file(&String::from("medium_puzzle.txt"));
    // let mut puzzle = Puzzle::from_file(&String::from("difficult_puzzle.txt")); // This one takes a while!
    let mut locations = Vec::<CellLocation>::with_capacity(9 * 9);
    let mut domains = HashMap::<CellLocation, Vec<u8>>::with_capacity(9 * 9);
    for (row_index, row) in puzzle.grid.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            let cell_location = CellLocation {
                row: row_index as u8,
                column: column_index as u8,
            };
            locations.push(cell_location.clone());
            if *value == 0 {
                domains.insert(cell_location, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            } else {
                domains.insert(cell_location, vec![*value]);
            }
        }
    }

    let mut csp = csp::CSP::<CellLocation, u8, SudokuConstraint>::new(domains);
    csp.add_constraint(SudokuConstraint::new(locations));

    let solution = csp.backtracking_search();
    match solution {
        None => println!("No solution found!"),
        Some(solution) => {
            puzzle.update(&solution);
            puzzle.display();
        }
    }
}
