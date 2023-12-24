// map_coloring.rs
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

struct MapColoringConstraint {
    region1: &'static str,
    region2: &'static str,
    variables: Vec<&'static str>
}

impl MapColoringConstraint {
    fn new(region1: &'static str, region2: &'static str) -> Self {
        MapColoringConstraint {region1, region2, variables: vec![region1, region2]}
    }
}

impl csp::Constraint<&str, &str> for MapColoringConstraint {
    fn satisfied(&self, assignment: &HashMap<Rc<&str>, &str>) -> bool {
        // If either region is not in the assignment then it is not
        // yet possible for their colors to be conflicting
        if !assignment.contains_key(&self.region1) || !assignment.contains_key(&self.region2) {
            return true;
        }
        assignment.get(&self.region1).unwrap() != assignment.get(&self.region2).unwrap()
    }
    fn variables<'a>(&'a self) -> &'a Vec<&'static str> {
        &self.variables
    }
}

fn main() {
    let colors = vec!["red","green","blue"];
    let mut domains = HashMap::<&str, Vec<&str>>::new();
    for variable in &vec![
        "WesternAustralia",
        "NorthernTerritory",
        "SouthAustralia",
        "Queensland",
        "NewSouthWales",
        "Victoria",
        "Tasmania"] {
        domains.insert(variable, colors.clone());
    }
    let mut csp = csp::CSP::<&str, &str, MapColoringConstraint>::new(domains);
    csp.add_constraint(MapColoringConstraint::new("WesternAustralia","NorthernTerritory"));
    csp.add_constraint(MapColoringConstraint::new("WesternAustralia","SouthAustralia"));
    csp.add_constraint(MapColoringConstraint::new("SouthAustralia","NorthernTerritory"));
    csp.add_constraint(MapColoringConstraint::new("Queensland","NorthernTerritory"));
    csp.add_constraint(MapColoringConstraint::new("Queensland", "SouthAustralia"));
    csp.add_constraint(MapColoringConstraint::new("Queensland", "NewSouthWales"));
    csp.add_constraint(MapColoringConstraint::new("NewSouthWales","SouthAustralia"));
    csp.add_constraint(MapColoringConstraint::new("Victoria", "SouthAustralia"));
    csp.add_constraint(MapColoringConstraint::new("Victoria", "NewSouthWales"));
    csp.add_constraint(MapColoringConstraint::new("Victoria", "Tasmania"));

    let solution = csp.backtracking_search();
    match solution {
        None => println!("No solution found!"),
        Some(solution) => println!("{:#?}", solution),
    }
}
