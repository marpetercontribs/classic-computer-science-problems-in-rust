// csp/src/lib.rs
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

use std::collections::HashMap;
use std::hash::Hash;

// Rust doesn't have abstract classes or method overloading like usual OO languages
// But structures and traits --> Holding a constraint's data must be in in struct
// and the "abstract" method defined as a trait
trait Satisfied {
    fn satisfied<V,D>(&self, assignment: &HashMap<V,D>) -> bool;
}

pub struct Constraint<V: Eq + Hash> {
    variables: Vec<V>,
//    satisfied: Box<dyn Fn(&HashMap<V,D>)-> bool>,
}

impl<V: Eq + Hash> Constraint<V> {
    pub fn new(variables: Vec<V> ) -> Self {
        Constraint { variables}
    }
}

pub struct CSP<V: Eq + Hash,D> {
    variables: Vec<V>,
    domains:   HashMap<V,Vec<D>>,
    constraints: HashMap<V,Vec<Constraint<V>>>
}

impl<V: Eq + Hash + Copy,D> CSP<V,D> {
    pub fn new(variables: Vec<V>, domains: HashMap<V,Vec<D>>) -> Self {
        let mut constraints = HashMap::<V,Vec<Constraint<V>>>::new();
        for variable in &variables {
            constraints.insert(*variable,Vec::<Constraint<V>>::new());
            if !domains.contains_key(variable) {
                panic!("Every variable should have a domain assigned to it.");
            }
        }
        CSP {
            variables,
            domains,
            constraints
        }
    }

    pub fn add_constraint(&self, constraint: Constraint<V>) {
        for variable in &constraint.variables {
            if !self.variables.contains(&variable) {
                panic!("Variable in constraint not in CSP");
            } else {
                self.constraints.get(&variable).expect("Variable in constraint not in CSP").push(constraint);
            }
        }
    }

    // Check if the value assignment is consistent by checking all constraints
    // for the given variable against it
    pub fn is_consistent(&self, variable: V, assignment: HashMap<V, D> ) -> bool {
        while let Some(constraint) = self.constraints.get(&variable) {
            if !constraint.satisfied(assignment) {
                return false;
            }
        }
        true
    }
}
