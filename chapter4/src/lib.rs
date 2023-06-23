// lib.rs
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

pub mod edge;
pub mod graph;
pub mod unweighted_graph;
pub mod weighted_graph;

pub fn vec_to_string<V: ToString>(list: &[V]) -> String {
    let mut result = String::from("[");
    for s in list.iter().map(|v| v.to_string()) {
        result.push_str(&format!("{s}, "));
    }
    result.pop();
    result.pop();
    result.push(']');
    result
}

pub fn tuple_vec_to_string<V: ToString, S: ToString>(list: &[(V, S)]) -> String {
    let mut result = String::from("[");
    for (vs, ss) in list.iter().map(|(v, s)| (v.to_string(), s.to_string())) {
        result.push_str(&format!("({vs}, {ss}), "));
    }
    result.pop();
    result.pop();
    result.push(']');
    result
}
