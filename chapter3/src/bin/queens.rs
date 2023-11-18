// queens.rs
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

use std::collections::HashMap;
use std::rc::Rc;

struct QueensConstraint {
    columns: Vec<u8>,
}

impl QueensConstraint {
    fn new(columns: Vec<u8>) -> Self {
        QueensConstraint { columns }
    }
}

impl csp::Constraint<u8, u8> for QueensConstraint {
    fn satisfied(&self, assignment: &HashMap<Rc<u8>, u8>) -> bool {
        for (queen1_column, queen1_row) in assignment.iter() {
            for queen2_column in (queen1_column.as_ref() + 1)..(self.columns.len() + 1).try_into().unwrap() {
                if let Some(queen2_row) = assignment.get(&queen2_column) {
                    if queen1_row == queen2_row {
                        // same row
                        return false;
                    }
                    if queen2_column.abs_diff(*queen1_column.as_ref()) == queen1_row.abs_diff(*queen2_row) {
                        // same diagonal
                        return false;
                    }
                }
            }
        }
        true
    }
    fn variables(&self) -> Vec<u8> {
        self.columns.clone()
    }
}

fn main() {
    let columns = Vec::from([1, 2, 3, 4, 5, 6, 7, 8]); // the "variables"
    let mut rows = HashMap::<u8, Vec<u8>>::new(); // the "domains"
    for column in &columns {
        rows.insert(*column, Vec::from([1, 2, 3, 4, 5, 6, 7, 8]));
    }

    let mut csp = csp::CSP::<u8, u8, QueensConstraint>::new(rows);
    csp.add_constraint(QueensConstraint::new(columns));
    let solution = csp.backtracking_search();
    match solution {
        None => println!("No solution found!"),
        Some(solution) => println!("{:#?}", solution),
    }
}
