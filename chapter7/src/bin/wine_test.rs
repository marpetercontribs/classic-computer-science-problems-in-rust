// wine_test.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 7
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
use chapter7::network::Network;
use chapter7::utils::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::rc::Rc;

fn interpret_output(output: &[f64]) -> usize {
    let max_output = max(output);
    if max_output == output[0] {
        return 1;
    } else if max_output == output[1] {
        return 2;
    }
    3
}

fn main() {
    let mut wine_parameters: Vec<Vec<f64>> = Vec::new();
    let mut wine_classifications: Vec<Vec<f64>> = Vec::new();
    let mut wine_species: Vec<usize> = Vec::new();

    let mut dataset: Vec<Vec<String>> = load_csv("src/wine.csv");
    dataset.shuffle(&mut thread_rng());

    for wine in dataset.iter() {
        let parameters: Vec<f64> = wine
            .iter()
            .skip(1)
            .map(|value| value.parse().expect("No f64"))
            .collect();
        wine_parameters.push(parameters);
        let species: usize = wine[0].parse().expect("No int");
        if species == 1 {
            wine_classifications.push(vec![1_f64, 0_f64, 0_f64]);
        } else if species == 2 {
            wine_classifications.push(vec![0_f64, 1_f64, 0_f64]);
        } else {
            wine_classifications.push(vec![0_f64, 0_f64, 1_f64]);
        }
        wine_species.push(species);
    }
    normalize_by_feature_scaling(&mut wine_parameters);

    let wine_network: Network = Network::new(
        vec![13, 7, 3],
        0.9,
        Rc::new(sigmoid),
        Rc::new(sigmoid_derivative),
    );

    // train over the first 150 wines in the data set 10 times
    let trainers = &wine_parameters[0..150];
    let trainers_corrects = &wine_classifications[0..150];
    for _ in 0..10 {
        wine_network.train(trainers, trainers_corrects);
    }
    // test over the last wines in the data set
    let testers = &wine_parameters[150..];
    let testers_corrects = &wine_species[150..];
    let results = wine_network.validate(testers, testers_corrects, &interpret_output);

    println!(
        "{} correct of {} = {}%",
        results.0,
        results.1,
        results.2 * 100_f64
    );
}
