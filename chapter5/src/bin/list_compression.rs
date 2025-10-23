// list_compression.rs
// Adapted for Rust version
// of Classic Computer Science Problems in Python/Java Chapter 5
//
// This code does not directly correspond to any of the problems covered in
// the books, but to have a file structure similar to Kopec's (on github)
// compatible with rust's package manager cargo, with one folder per chapter
// without having to change the generated Cargo.toml for each program
//   of chapter 1 in the books, and
// without having a seperate rust package and sub-folder for each program
//   with its own src folder and generic file name main.rs
//
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
use chapter5::chromosome::Chromosome;
use chapter5::genetic_algorithm::GeneticAlgorithm;
use chapter5::genetic_algorithm::SelectionType;
use libflate::gzip::Encoder;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::Write;

#[derive(Clone)]
struct ListCompression {
    list: Vec<String>,
}

impl ListCompression {
    fn bytes_compressed(&self) -> usize {
        // not sure if this is really equivalent to what's used in Kopec's books ...
        let mut encoder = Encoder::new(Vec::new()).unwrap();
        for s in self.list.iter() {
            let _ = encoder.write(&s.clone().into_bytes());
        }
        encoder.finish().into_result().unwrap().len()
    }
    fn index_of(&self, string: &str) -> usize {
        self.list.iter().position(|e| e == string).unwrap()
    }
}

impl Chromosome for ListCompression {
    fn fitness(&self) -> f64 {
        1.0 / (self.bytes_compressed() as f64)
    }
    fn crossover(&self, that: &ListCompression) -> (Self, Self) {
        let mut child1 = self.clone();
        let mut child2 = that.clone();
        let mut rng = rand::rng();
        let idx1 = rng.random_range(0..child1.list.len());
        let idx2 = rng.random_range(0..child2.list.len());
        let string1 = child1.list[idx1].clone();
        let string2 = child2.list[idx2].clone();
        let idx3 = child1.index_of(&string2);
        let idx4 = child2.index_of(&string1);
        (child1.list[idx1], child1.list[idx3]) =
            (child1.list[idx3].clone(), child1.list[idx1].clone());
        (child2.list[idx2], child2.list[idx4]) =
            (child2.list[idx4].clone(), child2.list[idx2].clone());
        (child1, child2)
    }
    fn mutate(&mut self) {
        let mut rng = rand::rng();
        let idx1 = rng.random_range(0..self.list.len());
        let idx2 = rng.random_range(0..self.list.len());
        (self.list[idx1], self.list[idx2]) = (self.list[idx2].clone(), self.list[idx1].clone());
    }
    fn random_instance() -> Self {
        let mut list: Vec<String> = vec![
            "Michael", "Sarah", "Joshua", "Narine", "David", "Sajid", "Melanie", "Daniel", "Wei",
            "Dean", "Brian", "Murat", "Lisa",
        ]
        .iter()
        .map(|str| str.to_string())
        .collect();
        list.shuffle(&mut rand::rng());
        ListCompression { list }
    }
}

impl std::fmt::Display for ListCompression {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("[");
        for s in self.list.iter() {
            result.push_str(&format!("{s}, "));
        }
        result.pop();
        result.pop();
        result.push(']');
        write!(formatter,"Order: {} Bytes: {}", result, self.bytes_compressed())
    }
}

fn main() {
    let mut initial_population: Vec<ListCompression> = Vec::new();
    for _ in 0..100 {
        initial_population.push(ListCompression::random_instance());
    }
    let mut ga: GeneticAlgorithm<ListCompression> =
        GeneticAlgorithm::new(initial_population, 0.2, 0.7, SelectionType::Tournament);
    let result: ListCompression = ga.run(100, 1.0);
    println!("{result}");
}
