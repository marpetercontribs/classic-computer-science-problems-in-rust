// data_point.rs
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
use std::fmt;
use std::iter::zip;

pub trait DataPoint: Clone {
    fn originals(&self) -> Vec<f64>;
    fn coordinates(&self) -> Vec<f64>;
    fn set_coordinates(&mut self, coordinates: Vec<f64>);
    fn num_dimensions(&self) -> usize;
    fn distance<P: DataPoint>(&self, other: &P) -> f64 {
        let combined: f64 = zip(self.coordinates().iter(), other.coordinates().iter())
            .map(|(x, y)| (x - y) * (x - y))
            .sum();
        combined.sqrt()
    }
}

#[derive(Clone)]
pub struct SimpleDataPoint {
    originals: Vec<f64>,
    coordinates: Vec<f64>,
    num_dimensions: usize,
}

impl SimpleDataPoint {
    pub fn new(initials: Vec<f64>) -> Self {
        SimpleDataPoint {
            originals: initials.clone(),
            num_dimensions: initials.len(),
            coordinates: initials,
        }
    }
}
impl fmt::Debug for SimpleDataPoint {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.originals())
    }
}

impl DataPoint for SimpleDataPoint {
    fn originals(&self) -> Vec<f64> {
        self.originals.clone()
    }
    fn coordinates(&self) -> Vec<f64> {
        self.coordinates.clone()
    }
    fn set_coordinates(&mut self, coordinates: Vec<f64>) {
        self.coordinates = coordinates;
    }
    fn num_dimensions(&self) -> usize {
        self.num_dimensions
    }
    // fn distance<P: DataPoint>(&self, other: &P) -> f64 {
    //     let combined: f64 = zip(self.coordinates.iter(), other.coordinates().iter())
    //         .map(|(x, y)| (x - y) * (x - y))
    //         .sum();
    //     combined.sqrt()
    // }
}
