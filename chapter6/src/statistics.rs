// statistics.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 6
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

pub struct Statistics {
    pub max: f64,
    pub min: f64,
    pub sum: f64,
    pub mean: f64,
    pub std: f64,
}

impl Statistics {
    pub fn new(values: &[f64]) -> Self {
        let min = values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .clone();
        let max = values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .clone();
        let sum: f64 = values.iter().sum();
        let mean = sum / (values.len() as f64); // use crate conv instead?
        let variance: f64 = values
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / (values.len() as f64); // use crate conv instead?
        Statistics {
            max,
            min,
            sum,
            mean,
            std: variance.sqrt(),
        }
    }
    pub fn zscore(values: &[f64]) -> Vec<f64> {
        let stats = Statistics::new(values);
        values
            .iter()
            .map(|value| {
                if stats.std != 0.0 {
                    (value - stats.mean) / stats.std
                } else {
                    0.0
                }
            })
            .collect()
    }
}
