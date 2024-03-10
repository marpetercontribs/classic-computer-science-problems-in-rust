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

// Rust does not support inheritance for structs, and traits cannot contain data.
// Thus we cannot make the things to cluster inherit from a DataPoint struct
// But all we need for the cluster coordinates is the ability to convert the things
// to cluster into DataPoints
// => Use a concrete implementation of DataPoint, not just a trait
//    base Cluster and KMeans on things that implement Into<DataPoint>
//    requires separating the "originals" from the "z_scored" coordinates

#[derive(Clone)]
pub struct DataPoint<P: Into<DataPoint<P>> + Clone + fmt::Debug> {
    pub original: P,
    pub coordinates: Vec<f64>,
    pub num_dimensions: usize,
}

impl From<Vec<f64>> for DataPoint<Vec<f64>> {
    fn from(item: Vec<f64>) -> Self {
        Self {
            num_dimensions: item.len(),
            original: item.clone(),
            coordinates: item,
        }
    }
}

impl<P: Into<DataPoint<P>> + Clone + fmt::Debug> DataPoint<P> {
    pub fn original(&self) -> P {
        self.original.clone()
    }
    pub fn coordinates(&self) -> Vec<f64> {
        self.coordinates.clone()
    }
    pub fn set_coordinates(&mut self, coordinates: Vec<f64>) {
        self.coordinates = coordinates;
    }
    pub fn num_dimensions(&self) -> usize {
        self.num_dimensions
    }
    pub fn distance(&self, other: &DataPoint<Vec<f64>>) -> f64 {
        let combined: f64 = zip(self.coordinates().iter(), other.coordinates().iter())
            .map(|(x, y)| (x - y) * (x - y))
            .sum();
        combined.sqrt()
    }
}

impl<P: Into<DataPoint<P>> + Clone + fmt::Debug> fmt::Debug for DataPoint<P> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.original)
    }
}
