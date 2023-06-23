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

pub trait Edge {
    fn reversed(&self) -> Self where Self: Sized;
}

#[derive(Clone)]
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
}

impl ToString for SimpleEdge {
    fn to_string(&self) -> String {
        format!("{} -> {}", self.u, self.v)
    }
}
