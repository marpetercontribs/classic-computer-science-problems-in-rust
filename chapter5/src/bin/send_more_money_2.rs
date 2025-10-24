// send_more_money_2.rs
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
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone)]
struct SendMoreMoney {
    letters: Vec<char>,
}

impl SendMoreMoney {
    fn index_of(&self, letter: char) -> usize {
        self.letters.iter().position(|&e| e == letter).unwrap()
    }
    fn send(&self) -> usize {
        let s = self.index_of('S');
        let e = self.index_of('E');
        let n = self.index_of('N');
        let d = self.index_of('D');
        1000 * s + 100 * e + 10 * n + d
    }
    fn more(&self) -> usize {
        let m = self.index_of('M');
        let o = self.index_of('O');
        let r = self.index_of('R');
        let e = self.index_of('E');
        1000 * m + 100 * o + 10 * r + e
    }
    fn money(&self) -> usize {
        let m = self.index_of('M');
        let o = self.index_of('O');
        let n = self.index_of('N');
        let e = self.index_of('E');
        let y = self.index_of('Y');
        10000 * m + 1000 * o + 100 * n + 10 * e + y
    }
}

impl Chromosome for SendMoreMoney {
    fn fitness(&self) -> f64 {
        let send: usize = self.send();
        let more: usize = self.more();
        let money: usize = self.money();
        let difference: isize = (money as isize - ((send + more) as isize)).abs();
        1.0 / (1.0 + (difference as f64))
    }
    fn crossover(&self, that: &SendMoreMoney) -> (Self, Self) {
        let mut child1 = self.clone();
        let mut child2 = that.clone();
        let mut rng = rand::rng();
        let idx1 = rng.random_range(0..child1.letters.len());
        let idx2 = rng.random_range(0..child2.letters.len());
        let letter1 = child1.letters[idx1];
        let letter2 = child2.letters[idx2];
        let idx3 = child1.index_of(letter2);
        let idx4 = child2.index_of(letter1);
        (child1.letters[idx1], child1.letters[idx3]) = (child1.letters[idx3], child1.letters[idx1]);
        (child2.letters[idx2], child2.letters[idx4]) = (child2.letters[idx4], child2.letters[idx2]);
        (child1, child2)
    }
    fn mutate(&mut self) {
        let mut rng = rand::rng();
        let idx1 = rng.random_range(0..self.letters.len());
        let idx2 = rng.random_range(0..self.letters.len());
        (self.letters[idx1], self.letters[idx2]) = (self.letters[idx2], self.letters[idx1]);
    }
    fn random_instance() -> Self {
        let mut letters = vec!['S', 'E', 'N', 'D', 'M', 'O', 'R', 'Y', ' ', ' '];
        letters.shuffle(&mut rand::rng());
        SendMoreMoney { letters }
    }
}

impl std::fmt::Display for SendMoreMoney {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let send: usize = self.send();
        let more: usize = self.more();
        let money: usize = self.money();
        let difference: isize = (money as isize - ((send + more) as isize)).abs();
        write!(formatter,"{send} + {more} = {money} Difference: {difference}")
    }
}

fn main() {
    let mut initial_population: Vec<SendMoreMoney> = Vec::new();
    for _ in 0..1000 {
        initial_population.push(SendMoreMoney::random_instance());
    }
    let mut ga: GeneticAlgorithm<SendMoreMoney> =
        GeneticAlgorithm::new(initial_population, 0.2, 0.7, SelectionType::Roulette);
    let result: SendMoreMoney = ga.run(1000, 1.0);
    println!("{result}");
}
