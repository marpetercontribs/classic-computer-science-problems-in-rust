// send_more_money.rs
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
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

struct SendMoreMoneyConstraint {
    letters: Vec<char>,
}

impl SendMoreMoneyConstraint {
    fn new(letters: Vec<char>) -> Self {
        SendMoreMoneyConstraint { letters }
    }
}

impl csp::Constraint<char, u16> for SendMoreMoneyConstraint {
    fn satisfied(&self, assignment: &HashMap<Rc<char>, u16>) -> bool {
        let assignment_values: HashSet<&u16> = HashSet::from_iter(assignment.values());
        if assignment_values.len() < assignment.len() {
            // if there are duplicate values then it's not a solution
            return false;
        } else if assignment.len() == self.letters.len() {
            // if all variables have been assigned, check if it adds correctly
            let s: u16 = *assignment.get(&'S').expect("S assignment missing");
            let e: u16 = *assignment.get(&'E').expect("E assignment missing");
            let n: u16 = *assignment.get(&'N').expect("N assignment missing");
            let d: u16 = *assignment.get(&'D').expect("D assignment missing");
            let m: u16 = *assignment.get(&'M').expect("M assignment missing");
            let o: u16 = *assignment.get(&'O').expect("O assignment missing");
            let r: u16 = *assignment.get(&'R').expect("R assignment missing");
            let y: u16 = *assignment.get(&'Y').expect("Y assignment missing");
            let send: u16 = 1000 * s + 100 * e + 10 * n + d;
            let more: u16 = 1000 * m + 100 * o + 10 * r + e;
            let money: u16 = 10000 * m + 1000 * o + 100 * n + 10 * e + y;
            return send + more == money;
        }
        true // no conflict
    }

    fn variables(&self) -> Vec<char> {
        self.letters.clone()
    }
}

fn main() {
    let letters = vec!['S', 'E', 'N', 'D', 'M', 'O', 'R', 'Y'];
    let mut possible_digits: HashMap<char, Vec<u16>> = HashMap::new();
    for letter in &letters {
        possible_digits.insert(*letter, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    let m_digits = possible_digits.get_mut(&'M').unwrap();
    *m_digits = vec![1]; // so we don't get answers starting with a 0

    let mut csp =
        csp::CSP::<char, u16, SendMoreMoneyConstraint>::new(possible_digits);
    csp.add_constraint(SendMoreMoneyConstraint::new(letters));
    let solution = csp.backtracking_search();
    match solution {
        None => println!("No solution found!"),
        Some(solution) => println!("{:#?}", solution),
    }
}
