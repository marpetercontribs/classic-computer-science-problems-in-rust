// util.rs
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
use std::fs::File;
use std::iter::zip;

pub fn dot_product(xs: &[f64], ys: &[f64]) -> f64 {
    zip(xs, ys).map(|(x, y)| x * y).sum::<f64>()
}

pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub fn sigmoid_derivative(x: f64) -> f64 {
    let sigmoid = sigmoid(x);
    sigmoid * (1.0 - sigmoid)
}

pub fn normalize_by_feature_scaling(dataset: &mut [Vec<f64>]) {
    for column_number in 0..dataset[0].len() {
        let column: Vec<f64> = dataset.iter().map(|row| row[column_number]).collect();
        let maximum = max(&column);
        let minimum = column.iter().fold(column[0], |min, value| min.min(*value));
        let difference = maximum - minimum;
        for row in dataset.iter_mut() {
            row[column_number] = (row[column_number] - minimum) / difference;
        }
    }
}

pub fn max(floats: &[f64]) -> f64 {
    floats.iter().fold(floats[0], |max, value| max.max(*value))
}

pub fn load_csv(file_name: &str) -> Vec<Vec<String>> {
    let file = File::open(file_name).expect("Could not open the csv file");
    let mut rdr = csv::Reader::from_reader(file);
    let mut result: Vec<Vec<String>> = Vec::new();
    for record in rdr.deserialize() {
        let line: Vec<String> = record.expect("Could not parse a line in the csv file");
        result.push(line);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        assert_eq!(dot_product(&vec![1.0, 0.0, 0.0], &vec![0.0, 1.0, 1.0]), 0.0);
        assert_eq!(dot_product(&vec![1.0, 1.0, 0.0], &vec![0.0, 1.0, 1.0]), 1.0);
        assert_eq!(dot_product(&vec![1.0, 2.0, 3.0], &vec![0.0, 1.0, 1.0]), 5.0);
    }

    #[test]
    fn test_sigmoid() {
        assert_eq!(sigmoid(0.0), 0.5);
    }
}
