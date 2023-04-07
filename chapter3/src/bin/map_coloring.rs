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
use csp;
use std::collections::HashMap;
use std::fmt;

// The original book uses simple strings for the domain values, but using an enum
// seems the better pattern and easier in Rust as String does not implement Copy
#[derive(Clone,Copy, PartialEq)]
enum Color { Red, Green, Blue }
impl fmt::Debug for Color {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red   => write!(formatter, "red"),
            Color::Green => write!(formatter, "green"),
            Color::Blue  => write!(formatter, "blue")
        }
    }
}

// The original book uses simple strings for the variables (names), but using an enum
// seems the better pattern and easier in Rust as String does not implement Copy
#[derive(Clone,Copy, PartialEq, Eq, Hash)]
enum Variable {
    WesternAustralia,
    NorthernTerritory,
    SouthAustralia,
    Queensland,
    NewSouthWales,
    Victoria,
    Tasmania
}
impl fmt::Debug for Variable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::WesternAustralia => write!(formatter, "Western Australia"),
            Variable::NorthernTerritory => write!(formatter, "Northern Territory"),
            Variable::SouthAustralia => write!(formatter, "South Australia"),
            Variable::Queensland => write!(formatter, "Queensland"),
            Variable::NewSouthWales => write!(formatter, "New South Wales"),
            Variable::Victoria => write!(formatter, "Victoria"),
            Variable::Tasmania => write!(formatter, "Tasmania")
        }
    }
}

struct MapColoringConstraint {
    region1: Variable,
    region2: Variable,
}

impl MapColoringConstraint {
    fn new(region1: Variable, region2: Variable) -> Self {
        MapColoringConstraint {
            region1,
            region2,
        }
    }
}

impl csp::Constraint<Variable,Color> for MapColoringConstraint {
    fn satisfied(&self, assignment: &HashMap<Variable,Color>) -> bool {
        // If either region is not in the assignment then it is not
        // yet possible for their colors to be conflicting
        if !assignment.contains_key(&self.region1) || !assignment.contains_key(&self.region2) {
            return true;
        }
        assignment.get(&self.region1).unwrap() != assignment.get(&self.region2).unwrap()
    }
    fn variables(&self) -> Vec<Variable> {
        vec![self.region1, self.region2]
    }
}

fn main() {
    let variables = vec![Variable::WesternAustralia, Variable::NorthernTerritory, Variable::SouthAustralia,
    Variable::Queensland, Variable::NewSouthWales, Variable::Victoria, Variable::Tasmania];
    let mut domains = HashMap::<Variable,Vec<Color>>::new();
    for variable in &variables {
        domains.insert(*variable,vec![Color::Red, Color::Green, Color::Blue]);
    }
    let mut csp = csp::CSP::<Variable,Color,MapColoringConstraint>::new(variables, domains);
    csp.add_constraint(MapColoringConstraint::new(Variable::WesternAustralia,Variable::NorthernTerritory).into());
    csp.add_constraint(MapColoringConstraint::new(Variable::WesternAustralia,Variable::SouthAustralia).into());
    csp.add_constraint(MapColoringConstraint::new(Variable::SouthAustralia,Variable::NorthernTerritory).into());
    csp.add_constraint(MapColoringConstraint::new(Variable::Queensland,Variable::NorthernTerritory).into());
    csp.add_constraint(MapColoringConstraint::new(Variable::Queensland,Variable::SouthAustralia).into());
    csp.add_constraint(MapColoringConstraint::new(Variable::Queensland,Variable::NewSouthWales).into());
    csp.add_constraint(MapColoringConstraint::new(Variable::NewSouthWales,Variable::SouthAustralia).into());
    csp.add_constraint(MapColoringConstraint::new(Variable::Victoria,Variable::SouthAustralia).into());
    csp.add_constraint(MapColoringConstraint::new(Variable::Victoria,Variable::NewSouthWales).into());
    csp.add_constraint(MapColoringConstraint::new(Variable::Victoria,Variable::Tasmania).into());

    let solution = csp.backtracking_search();
    match solution{
        None =>  println!("No solution found!"),
        Some(solution) => println!("{:#?}", solution)   
    }
 
}