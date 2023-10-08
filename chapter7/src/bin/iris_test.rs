// iris_test.rs
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

fn interpret_output(output: &[f64]) -> String {
    let max_output = max(output);
    if max_output == output[0] {
        return "Iris-setosa".to_string();
    } else if max_output == output[1] {
        return "Iris-versicolor".to_string();
    }
    "Iris-virginica".to_string()
}

fn main() {
    let mut iris_parameters: Vec<Vec<f64>> = Vec::new();
    let mut iris_classifications: Vec<Vec<f64>> = Vec::new();
    let mut iris_species: Vec<String> = Vec::new();

    let mut dataset: Vec<Vec<String>> = load_csv("src/iris.csv");
    dataset.shuffle(&mut thread_rng());

    for iris in dataset.iter() {
        let parameters: Vec<f64> = iris
            .iter()
            .take(4)
            .map(|value| value.parse().expect("No f64"))
            .collect();
        iris_parameters.push(parameters);
        let species = iris[4].clone();
        if species == "Iris-setosa" {
            iris_classifications.push(vec![1_f64, 0_f64, 0_f64]);
        } else if species == "Iris-versicolor" {
            iris_classifications.push(vec![0_f64, 1_f64, 0_f64]);
        } else {
            iris_classifications.push(vec![0_f64, 0_f64, 1_f64]);
        }
        iris_species.push(species);
    }
    normalize_by_feature_scaling(&mut iris_parameters);

    let iris_network: Network = Network::new(
        vec![4, 6, 3],
        0.3,
        Rc::new(sigmoid),
        Rc::new(sigmoid_derivative),
    );

    // train over the first 140 irises in the data set 50 times
    let trainers = &iris_parameters[0..140];
    let trainers_corrects = &iris_classifications[0..140];
    for _ in 0..50 {
        iris_network.train(trainers, trainers_corrects);
    }
    // test over the last 10 of the irises in the data set
    let testers = &iris_parameters[140..];
    let testers_corrects = &iris_species[140..];
    let results = iris_network.validate(testers, testers_corrects, &interpret_output);

    println!(
        "{} correct of {} = {}%",
        results.0,
        results.1,
        results.2 * 100_f64
    );
}
