// missionaries.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 2
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
use generic_search;

const MAX_NUM: usize = 3;

#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Copy)]
struct MCState {
    west_missionaries: usize,
    east_missionaries: usize,
    west_cannibals: usize,
    east_cannibals: usize,
    boat_on_west_bank: bool,
}

impl MCState {
    fn new(west_missionaries: usize, west_cannibals: usize, boat_on_west_bank: bool) -> Self {
        MCState {
            west_missionaries,
            east_missionaries: MAX_NUM - west_missionaries,
            west_cannibals,
            east_cannibals: MAX_NUM - west_cannibals,
            boat_on_west_bank,
        }
    }
    fn goal_test(&self) -> bool {
        self.is_legal() && self.east_missionaries == MAX_NUM && self.east_cannibals == MAX_NUM
    }
    fn is_legal(&self) -> bool {
        if self.west_missionaries > 0 && self.west_missionaries < self.west_cannibals {
            return false;
        }
        if self.east_missionaries > 0 && self.east_missionaries < self.east_cannibals {
            return false;
        }
        true
    }
    fn successors(&self) -> Vec<MCState> {
        let mut successors: Vec<MCState> = Vec::new();
        if self.boat_on_west_bank {
            if self.west_missionaries > 1 {
                successors.push(MCState::new(
                    self.west_missionaries - 2,
                    self.west_cannibals,
                    !self.boat_on_west_bank,
                ));
            }
            if self.west_missionaries > 0 {
                successors.push(MCState::new(
                    self.west_missionaries - 1,
                    self.west_cannibals,
                    !self.boat_on_west_bank,
                ));
            }
            if self.west_cannibals > 1 {
                successors.push(MCState::new(
                    self.west_missionaries,
                    self.west_cannibals - 2,
                    !self.boat_on_west_bank,
                ));
            }
            if self.west_cannibals > 0 {
                successors.push(MCState::new(
                    self.west_missionaries,
                    self.west_cannibals - 1,
                    !self.boat_on_west_bank,
                ));
            }
            if self.west_missionaries > 0 && self.west_cannibals > 0 {
                successors.push(MCState::new(
                    self.west_missionaries - 1,
                    self.west_cannibals - 1,
                    !self.boat_on_west_bank,
                ));
            }
        } else {
            if self.east_missionaries > 1 {
                successors.push(MCState::new(
                    self.west_missionaries + 2,
                    self.west_cannibals,
                    !self.boat_on_west_bank,
                ));
            }
            if self.east_missionaries > 0 {
                successors.push(MCState::new(
                    self.west_missionaries + 1,
                    self.west_cannibals,
                    !self.boat_on_west_bank,
                ));
            }
            if self.east_cannibals > 1 {
                successors.push(MCState::new(
                    self.west_missionaries,
                    self.west_cannibals + 2,
                    !self.boat_on_west_bank,
                ));
            }
            if self.east_cannibals > 0 {
                successors.push(MCState::new(
                    self.west_missionaries,
                    self.west_cannibals + 1,
                    !self.boat_on_west_bank,
                ));
            }
            if self.east_missionaries > 0 && self.east_cannibals > 0 {
                successors.push(MCState::new(
                    self.west_missionaries + 1,
                    self.west_cannibals + 1,
                    !self.boat_on_west_bank,
                ));
            }
        }
        successors.retain(|&state| state.is_legal());
        successors
    }
}

impl ToString for MCState {
    fn to_string(&self) -> String {
        let mut result = format!(
            "On the west bank there are {} missionaries and {} cannibals.\n",
            self.west_missionaries, self.west_cannibals
        );
        result.push_str(&format!(
            "On the east bank there are {} missionaries and {} cannibals.\n",
            self.east_missionaries, self.east_cannibals
        ));
        result.push_str(&format!(
            "The boat is on the {} bank.\n",
            if self.boat_on_west_bank {
                "west"
            } else {
                "east"
            }
        ));
        result
    }
}

fn display_solution(path: &Vec<MCState>) {
    if path.is_empty() {
        return;
    }
    let mut old_state = &path[0];
    println!("{}", old_state.to_string());
    for current_state in &path[1..] {
        if current_state.boat_on_west_bank {
            println!(
                "{} missionaries and {} cannibals moved from the east bank to the west bank.",
                old_state.east_missionaries - current_state.east_missionaries,
                old_state.east_cannibals - current_state.east_cannibals
            );
        } else {
            println!(
                "{} missionaries and {} cannibals moved from the west bank to the east bank.",
                old_state.west_missionaries - current_state.west_missionaries,
                old_state.west_cannibals - current_state.west_cannibals
            );
        }
        println!("{}", current_state.to_string());
        old_state = current_state;
    }
}

fn main() {
    let start = MCState::new(MAX_NUM, MAX_NUM, true);
    let solution = generic_search::bfs(
        start,
        |&state| state.goal_test(),
        |&state| state.successors(),
    );
    match solution {
        None => println!("No solution found!"),
        Some(node) => {
            let path = generic_search::node_to_path(&node);
            display_solution(&path);
        }
    }
}
