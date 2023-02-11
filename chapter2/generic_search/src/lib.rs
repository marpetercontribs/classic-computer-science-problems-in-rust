//generic_search.rs
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
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

pub fn linear_contains<'a, T: 'a + PartialEq>( iterable: impl IntoIterator<Item = &'a T>, key: &T) -> bool {
    for item in iterable.into_iter() {
        if item == key {
            return true;
        }
    }
    false
}

pub fn binary_contains<'a, T: 'a + PartialOrd + ?Sized>( list: &Vec<&'a T>, key: &T) -> bool {
    let mut low: usize = 0;
    let mut high: usize = list.len() - 1;
    while low <= high {
        let middle = (low + high)/2;
        if list[middle] < key {
            low = middle+1;
        } else if list[middle] > key {
            high = middle-1;
        } else {
            return true;
        }
    }
    false
}

#[derive(PartialEq, Clone)]
pub struct Node<T: PartialEq + Clone> {
    state: T,
    parent: Option<Rc<Node<T>>>, // indirection because of recursive type definition
    cost: f64,
    heuristic: f64,
}

impl<T: PartialEq + Clone> Node<T> {
    fn new(state: T, parent_node: Option<Rc<Node<T>>>) -> Self {
        Self::new_with_cost(state, parent_node, 0.0, 0.0)
    }
    fn new_with_cost(state: T, parent_node: Option<Rc<Node<T>>>, cost: f64, heuristic: f64) -> Self {
        match parent_node {
            None => Node { state, parent: None, cost, heuristic},
            Some(parent) => Node { state, parent: Some(Rc::clone(&parent)), cost, heuristic} 
        }
              
    }
}

impl<'a, T: PartialEq + Clone> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.cost + self.heuristic).partial_cmp(&(&other.cost + &other.heuristic))
    }
}

pub fn node_to_path<T: PartialOrd + Copy>(node: &Node<T>) -> Vec<T> {
    let mut path =Vec::<T>::new();
    let mut current_node = node;
    path.push(current_node.state);
    while let Some(parent) = &current_node.parent {
        current_node = &parent;
        path.push(current_node.state);
    }
    path
}

pub fn dfs<'a, T: PartialOrd + Copy + Eq + Hash, GT: Fn(&T) -> bool, S: Fn(&T) -> Vec<T>>(initial: T, goal_test: GT, successors: S) -> Option<Rc<Node<T>>> {
    // frontier is where we've yet to go
    let mut frontier = Vec::<Rc<Node<T>>>::new();
    let mut explored = HashSet::<T>::new();
    frontier.push(Rc::new(Node::<T>::new(initial,None)));
    explored.insert(initial);
    while let Some(current_node) = frontier.pop() {
        let current_state = current_node.state;
        // if we found the goal, we're done
        if goal_test(&current_state) {
            return Some(current_node);
        }
        // check where we can go next and haven't explored
        for child in successors(&current_state) {
            if !explored.contains(&child) {
                explored.insert(child);
                frontier.push(Rc::new(Node::<T>::new(child,Some(Rc::clone(&current_node)))));
            }
        }
    }
    None // went through everything and never found goal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_contains_works() {
        assert_eq!( linear_contains(&vec!(1, 5, 15, 15, 15, 15, 20),&15), true );
        assert_eq!( linear_contains(&vec!(1, 5, 15, 15, 15, 15, 20),&6), false );
    }

    #[test]
    fn binary_contains_works() {
        assert_eq!( binary_contains(&vec!("a", "d", "e", "f", "z"),"f"), true);
        assert_eq!( binary_contains(&vec!("john", "mark", "ronald", "sarah"),"sheila"), false);
    }
}
