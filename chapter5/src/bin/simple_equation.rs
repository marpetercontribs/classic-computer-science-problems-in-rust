// simple_equation.rs
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
use rand::{random, Rng};

#[derive(Clone)]
struct SimpleEquation {
    x: i64,
    y: i64,
}

impl Chromosome for SimpleEquation {
    fn fitness(&self) -> f64 {
        (6 * self.x - self.x * self.x + 4 * self.y - self.y * self.y) as f64
    }
    fn crossover(&self, that: &SimpleEquation) -> (Self, Self) {
        let mut child1 = self.clone();
        let mut child2 = that.clone();
        child1.y = child2.y;
        child2.y = self.y;
        (child1, child2)
    }
    fn mutate(&mut self) {
        if random::<f64>() > 0.5 {
            if random::<f64>() > 0.5 {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        } else if random::<f64>() > 0.5 {
            self.y += 1;
        } else {
            self.y -= 1;
        }
    }
    fn random_instance() -> Self {
        let mut rng = rand::rng();
        SimpleEquation {
            x: rng.random_range(0..100),
            y: rng.random_range(0..100),
        }
    }
}

impl std::fmt::Display for SimpleEquation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter,"X: {} Y: {} Fitness: {}", self.x, self.y, self.fitness())
    }
}

fn main() {
    let mut initial_population: Vec<SimpleEquation> = Vec::new();
    for _ in 0..20 {
        initial_population.push(SimpleEquation::random_instance());
    }
    let mut ga: GeneticAlgorithm<SimpleEquation> =
        GeneticAlgorithm::new(initial_population, 0.1, 0.7, SelectionType::ImprovedTournament);
    let result: SimpleEquation = ga.run(100, 13.0);
    println!("{result}");
}
