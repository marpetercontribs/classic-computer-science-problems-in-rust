// edge.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 4
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

pub trait Edge {
    fn reversed(&self) -> Self
    where
        Self: Sized;
    fn u(&self) -> usize;
    fn v(&self) -> usize;
}

#[derive(Clone, PartialEq)]
pub struct SimpleEdge {
    pub u: usize,
    pub v: usize,
}

impl SimpleEdge {
    pub fn new(u: usize, v: usize) -> Self {
        SimpleEdge { u, v }
    }
}

impl Edge for SimpleEdge {
    fn reversed(&self) -> SimpleEdge {
        SimpleEdge {
            u: self.v,
            v: self.u,
        }
    }
    fn u(&self) -> usize {
        self.u
    }
    fn v(&self) -> usize {
        self.v
    }
}

impl ToString for SimpleEdge {
    fn to_string(&self) -> String {
        format!("{} -> {}", self.u, self.v)
    }
}

#[derive(Clone, PartialEq)]
pub struct WeightedEdge {
    pub simple_edge: SimpleEdge,
    pub weight: f64,
}

impl WeightedEdge {
    pub fn new(u: usize, v: usize, weight: f64) -> Self {
        WeightedEdge {
            simple_edge: SimpleEdge { u, v },
            weight,
        }
    }
}

impl Edge for WeightedEdge {
    fn reversed(&self) -> WeightedEdge {
        WeightedEdge {
            simple_edge: self.simple_edge.reversed(),
            weight: self.weight,
        }
    }
    fn u(&self) -> usize {
        self.simple_edge.u
    }
    fn v(&self) -> usize {
        self.simple_edge.v
    }
}

impl ToString for WeightedEdge {
    fn to_string(&self) -> String {
        format!(
            "{} {} -> {}",
            self.simple_edge.u, self.weight, self.simple_edge.v
        )
    }
}

impl Ord for WeightedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        let distance = self.weight - other.weight;
        if distance < -0.000001_f64 {
            Ordering::Greater // Note the reversed order, because Rust's BinaryHeap pops highest priority element first!
        } else if distance > 0.000001_f64 {
            Ordering::Less // Note the reversed order, because Rust's BinaryHeap pops highest priority element first!
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for WeightedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for WeightedEdge {}
