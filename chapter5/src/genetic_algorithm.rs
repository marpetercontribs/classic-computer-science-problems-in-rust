// genetic_algorithm.rs
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

use crate::chromosome::Chromosome;
use rand::seq::SliceRandom;
use rand::random;

pub enum SelectionType {
    Roulette,
    Tournament,
    ImprovedTournament,
}

pub struct GeneticAlgorithm<C: Chromosome> {
    population: Vec<C>,
    mutation_chance: f64,
    crossover_chance: f64,
    selection_type: SelectionType,
}

impl<C: Chromosome> GeneticAlgorithm<C> {
    pub fn new(
        initial_population: Vec<C>,
        mutation_chance: f64,
        crossover_chance: f64,
        selection_type: SelectionType,
    ) -> Self {
        GeneticAlgorithm {
            population: initial_population,
            mutation_chance,
            crossover_chance,
            selection_type,
        }
    }

    fn pick_roulette(&self) -> (C, C) {
        let total_fitness: f64 = self.population.iter().map(|c| c.fitness()).sum();
        let wheel: Vec<f64> = self.population
            .iter()
            .map(|c| c.fitness() / total_fitness)
            .collect();
        let mut picks: Vec<C> = Vec::<C>::with_capacity(2);
        while picks.len() < 2 {
            let mut pick = random::<f64>();
            for (i, value) in wheel.iter().enumerate() {
                pick -= value;
                if pick <= 0_f64 {
                    picks.push(self.population[i].clone());
                    break;
                }
            }
        }
        (picks[0].clone(), picks[1].clone())
    }

    fn pick_tournament(&self, num_players: usize) -> (C, C) {
        let mut picks = self.pick_random_subset(num_players);
        picks.sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
        (picks[0].clone(), picks[1].clone())
    }

    fn pick_tournament_improved(&self, num_players: usize) -> (C, C) {
        let mut picks = self.pick_random_subset(num_players);
        picks.sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
        let total: f64 = picks.iter().take(3).map(|c| c.fitness()).sum();
        let first_two: f64 = (picks[0].fitness() + picks[1].fitness()) / total;
        let random = random::<f64>();
        if random < first_two {
            return (picks[0].clone(), picks[1].clone());
        } else {
            return (picks[1].clone(), picks[2].clone());
        }
    }

    fn pick_random_subset(&self, num_players: usize) -> Vec<C> {
        let mut picks: Vec<C> = self.population.clone();
        picks.shuffle(&mut rand::rng());
        picks.truncate(num_players);
        picks
    }

    fn reproduce_and_replace(&mut self) {
        let mut next_population: Vec<C> = Vec::<C>::new();
        while next_population.len() < self.population.len() {
            let (parent1, parent2): (C, C) = match self.selection_type {
                SelectionType::Roulette => self.pick_roulette(),
                SelectionType::Tournament => self.pick_tournament(self.population.len() / 2),
                SelectionType::ImprovedTournament => {
                    self.pick_tournament_improved(self.population.len() / 2)
                }
            };
            if random::<f64>() < self.crossover_chance {
                let (parent1, parent2) = parent1.crossover(&parent2);
                next_population.push(parent1);
                next_population.push(parent2);
            } else {
                next_population.push(parent1);
                next_population.push(parent2);
            }
        }
        if next_population.len() > self.population.len() {
            next_population.pop();
        }
        self.population = next_population;
    }
    fn mutate(&mut self) {
        for c in self.population.iter_mut() {
            if random::<f64>() < self.mutation_chance {
                c.mutate();
            }
        }
    }
    pub fn run(&mut self, max_generations: usize, threshold: f64) -> C {
        self.population
            .sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());
        let mut best = self.population[self.population.len() - 1].clone();

        for generation in 0..max_generations {
            if best.fitness() >= threshold {
                // early exit if we beat threshold
                return best;
            }
            let total_fitness: f64 = self.population.iter().map(|c| c.fitness()).sum();
            println!(
                "Generation {generation} Best {} Avg {}",
                best.fitness(),
                total_fitness / (self.population.len() as f64)
            );
            self.reproduce_and_replace();
            self.mutate();
            self.population
                .sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());
            let highest = self.population[self.population.len() - 1].clone();
            if highest.fitness() > best.fitness() {
                best = highest;
            }
        }
        best
    }
}
