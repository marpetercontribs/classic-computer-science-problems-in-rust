// generic_search/src/lib.rs
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
use core::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
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

pub fn binary_contains<'a, T: 'a + PartialOrd>( list: &Vec<T>, key: &T) -> bool {
    let mut low: usize = 0;
    let mut high: usize = list.len() - 1;
    while low <= high {
        let middle = (low + high) / 2;
        if list[middle] < *key {
            low = middle+1;
        } else if list[middle] > *key {
            if middle > 0 {
                high = middle-1;
            } else {
                return false;
            }
        } else {
            return true;
        }
    }
    false
}

#[derive(Clone)]
pub struct Node<T: PartialEq + Eq + Clone> {
    state: T,
    parent: Option<Rc<Node<T>>>, // indirection because of recursive type definition
    cost: f64, // Rust's f64 does not implement trait Eq!
    heuristic: f64,
}

impl<T: PartialEq + Eq + Clone> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}
impl<T: PartialEq + Eq + Clone> Eq for Node<T>  { }

impl<T: PartialEq + Eq + Clone> Node<T> {
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

impl<'a, T: PartialEq + Eq + Clone> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let difference = (self.cost + self.heuristic) - (&other.cost + &other.heuristic);
        if difference < -0.000001_f64 {
            Ordering::Greater // Note the reversed order, because Rust's BinaryHeap pops highest priority element first!
        } else if difference > 0.000001_f64  {
            Ordering::Less    // Note the reversed order, because Rust's BinaryHeap pops highest priority element first!
        } else {
            Ordering::Equal
        }
    }
}

impl<'a, T: PartialEq + Eq + Clone> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

pub fn node_to_path<T: PartialOrd + Eq + Copy>(node: &Node<T>) -> Vec<T> {
    let mut path =Vec::<T>::new();
    let mut current_node = node;
    path.push(current_node.state);
    while let Some(parent) = &current_node.parent {
        current_node = &parent;
        path.push(current_node.state);
    }
    println!("Path is {} steps long.",path.len());
    path.into_iter().rev().collect()
}

pub fn dfs<'a, T: PartialOrd + Copy + Eq + Hash, GT: Fn(&T) -> bool, S: Fn(&T) -> Vec<T>>(initial: T, goal_test: GT, successors: S) -> Option<Rc<Node<T>>> {
    let (result, _) = dfs_counting(initial, goal_test, successors);
    result
}

pub fn dfs_counting<'a, T: PartialOrd + Copy + Eq + Hash, GT: Fn(&T) -> bool, S: Fn(&T) -> Vec<T>>(initial: T, goal_test: GT, successors: S) -> (Option<Rc<Node<T>>>, usize) {
    // frontier is where we've yet to go
    let mut frontier = Vec::<Rc<Node<T>>>::new();
    let mut explored = HashSet::<T>::new();
    frontier.push(Rc::new(Node::<T>::new(initial,None)));
    explored.insert(initial);
    while let Some(current_node) = frontier.pop() {
        let current_state = current_node.state;
        // if we found the goal, we're done
        if goal_test(&current_state) {
            return (Some(current_node), explored.len());
        }
        // check where we can go next and haven't explored
        for child in successors(&current_state) {
            if !explored.contains(&child) {
                explored.insert(child);
                frontier.push(Rc::new(Node::<T>::new(child,Some(Rc::clone(&current_node)))));
            }
        }
    }
    (None, explored.len()) // went through everything and never found goal
}

pub fn bfs<'a, T: PartialOrd + Copy + Eq + Hash, GT: Fn(&T) -> bool, S: Fn(&T) -> Vec<T>>(initial: T, goal_test: GT, successors: S) -> Option<Rc<Node<T>>> {
    let (result, _) = bfs_counting(initial, goal_test, successors);
    result
}

pub fn bfs_counting<'a, T: PartialOrd + Copy + Eq + Hash, GT: Fn(&T) -> bool, S: Fn(&T) -> Vec<T>>(initial: T, goal_test: GT, successors: S) -> (Option<Rc<Node<T>>>, usize) {
    // frontier is where we've yet to go
    let mut frontier = VecDeque::<Rc<Node<T>>>::new();
    let mut explored = HashSet::<T>::new();
    frontier.push_back(Rc::new(Node::<T>::new(initial,None)));
    explored.insert(initial);
    while let Some(current_node) = frontier.pop_front() {
        let current_state = current_node.state;
        // if we found the goal, we're done
        if goal_test(&current_state) {
            return (Some(current_node), explored.len());
        }
        // check where we can go next and haven't explored
        for child in successors(&current_state) {
            if !explored.contains(&child) {
                explored.insert(child);
                frontier.push_back(Rc::new(Node::<T>::new(child,Some(Rc::clone(&current_node)))));
            }
        }
    }
    (None, explored.len()) // went through everything and never found goal
}

pub fn astar<'a, T: PartialOrd + Copy + Eq + Hash, GT: Fn(&T) -> bool, S: Fn(&T) -> Vec<T>, H: Fn(&T) -> f64>
    (initial: T, goal_test: GT, successors: S, heuristic: H) -> Option<Rc<Node<T>>> {
    let (result, _) = astar_counting(initial, goal_test, successors, heuristic);
    result
}

pub fn astar_counting<'a, T: PartialOrd + Copy + Eq + Hash, GT: Fn(&T) -> bool, S: Fn(&T) -> Vec<T>, H: Fn(&T) -> f64>
    (initial: T, goal_test: GT, successors: S, heuristic: H) -> (Option<Rc<Node<T>>>, usize) {
    // frontier is where we've yet to go
    let mut frontier = BinaryHeap::<Rc<Node<T>>>::new();
    let mut explored = HashMap::<T,f64>::new();
    frontier.push(Rc::new(Node::<T>::new_with_cost(initial,None, 0.0, heuristic(&initial))));
    explored.insert(initial, 0.0);
    while let Some(current_node) = frontier.pop() {
        let current_state = current_node.state;
        // if we found the goal, we're done
        if goal_test(&current_state) {
            return (Some(current_node), explored.len());
        }
        // check where we can go next and haven't explored
        for child in successors(&current_state) {
            let new_cost = current_node.cost + 1.0;
            if !explored.contains_key(&child) || explored.get(&child).unwrap() > &new_cost {
                explored.insert(child,new_cost);
                frontier.push(Rc::new(Node::<T>::new_with_cost(child,Some(Rc::clone(&current_node)), new_cost, heuristic(&child))));
            }
        }
    }
    (None, explored.len()) // went through everything and never found goal
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
        assert_eq!( binary_contains(&vec!(1, 5, 15, 15, 15, 15, 20),&15), true );
        assert_eq!( binary_contains(&vec!(1, 5, 15, 15, 15, 15, 20),&6), false );
        assert_eq!( binary_contains(&vec!("a".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "z".to_string()),&"f".to_string()), true);
        assert_eq!( binary_contains(&vec!("john".to_string(), "mark".to_string(), "ronald".to_string(), "sarah".to_string()),&"sheila".to_string()), false);
    }
}
