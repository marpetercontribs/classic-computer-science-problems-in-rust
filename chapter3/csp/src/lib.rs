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
use std::rc::Rc;

// Rust doesn't have abstract classes or method overloading like usual OO languages
// But structures and traits, where traits cannot hold any data
// --> Holding a constraint's data must be in in struct and the "abstract" methods
// defined as a trait
// Logically, "Contraint" seems to be the trait because that's what we want to "subclass"
// Issue: as soon as type parameters are introduced in the trait's method(s),
//   CSP cannot hold a HashMap of Contraints any longer: the compiler fails with 
//   "this trait cannot be made into an object..."
//   "...because method `satisfied` has generic type parameters"
//
// At least for the examples in the book, it seems to be sufficient if all
// Constraints held by the CSP are of the same "subtype" --> parameterize the trait
 
pub trait Constraint<V,D> {
    fn satisfied(&self, assignment: &HashMap<Rc<V>, D>) -> bool;
    fn variables<'a>(&'a self) -> &'a Vec<V>;
}

pub struct CSP<V: Eq + Hash,D: Clone, C: Constraint<V,D> + Sized> {
    // Because each variable must have a domain, we can use the keys of the domains HashMap as "variables"
    //   to avoid having to copy each variable into an explicit vector of variables
    // variables: Vec<V>,
    // The variables used as keys in domains and constraints are the same, and the same constraint is
    //   frequently shared among several variables,
    //   so use Reference Counting to avoid copying
    domains:   HashMap<Rc<V>,Vec<D>>,
    constraints: HashMap<Rc<V>,Vec<Rc<C>>>
}

impl<V: Eq + Hash + Clone, D: Clone, C: Constraint<V,D> + Sized> CSP<V,D,C> {
    pub fn new(domains_in: HashMap<V,Vec<D>>) -> Self {
        let mut constraints = HashMap::<Rc<V>,Vec<Rc<C>>>::new();
        let mut domains = HashMap::<Rc<V>,Vec<D>>::new();
        for (variable, domain) in domains_in {
            let rc_variable = Rc::new(variable);
            domains.insert(Rc::clone(&rc_variable), domain);
            constraints.insert(Rc::clone(&rc_variable),Vec::<Rc<C>>::new());
        }
        CSP {
            domains,
            constraints
        }
    }

    pub fn add_constraint(&mut self, constraint: C ) {
        let rc_constraint = Rc::new(constraint);
        for variable in rc_constraint.variables() {
            if !self.domains.contains_key(variable) {
                panic!("Variable in constraint not in CSP");
            } else {
                let constraints_for_var = self.constraints.get_mut(variable).expect("Variable in constraint not in CSP");
                constraints_for_var.push(Rc::clone(&rc_constraint));
            }
        }
    }

    // Check if the value assignment is consistent by checking all constraints
    // for the given variable against it
    pub fn is_consistent(&self, variable: &V, assignment: &HashMap<Rc<V>, D> ) -> bool {
        if let Some(constraint_vec) = self.constraints.get(variable) {
            for constraint in constraint_vec {
                if !constraint.satisfied(assignment) {
                    return false;
                }
            }
        }
        true
    }

    pub fn backtracking_search(&self) -> Option<HashMap<Rc<V>, D>> {
        self.internal_backtracking_search(HashMap::<Rc<V>,D>::new())
    }
    fn internal_backtracking_search(&self, assignment: HashMap<Rc<V>, D> ) -> Option<HashMap<Rc<V>, D>> {
        // assignment is complete if every variable is assigned (our base case)
        if assignment.len() == self.domains.len() {
            return Some(assignment);
        }
        // get all variables in the CSP but not in the assignment
        let unassigned_vars: Vec<&Rc<V>> = self.domains.keys().filter( |var| !assignment.contains_key(*var) ).collect();
        // get every possible domain value of the first unassigned variable
        let first = unassigned_vars[0];
        if let Some(values) = self.domains.get(first) {
            for value in values {
                let mut local_assignment = assignment.clone();
                local_assignment.insert(Rc::clone(first),(*value).clone());
                // if we're still consistent, we recurse (continue)
                if self.is_consistent(first, &local_assignment) {
                    let result = self.internal_backtracking_search(local_assignment);
                    if result.is_some() {
                        return result;
                    }
                }
            }
        }
        None
    }
}
