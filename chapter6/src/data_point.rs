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

pub trait DataPoint: Clone {
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
pub struct SimpleDataPoint<P: Into<SimpleDataPoint<P>> + Clone + fmt::Debug>{
    originals: P,
    coordinates: Vec<f64>,
    num_dimensions: usize,
}

impl From<Vec<f64>> for SimpleDataPoint<Vec<f64>> {
    fn from(item: Vec<f64>) -> Self {
        Self::new(item)
    }
}

impl SimpleDataPoint<Vec<f64>> {
    pub fn new(initials: Vec<f64>) -> Self {
        SimpleDataPoint {
            originals: initials.clone(),
            num_dimensions: initials.len(),
            coordinates: initials,
        }
    }
}

impl<P: Into<SimpleDataPoint<P>> + Clone + fmt::Debug> SimpleDataPoint<P> {
    pub fn originals(&self) -> P {
        self.originals.clone()
    }
}

impl<P: Into<SimpleDataPoint<P>> + Clone + fmt::Debug> fmt::Debug for SimpleDataPoint<P> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.originals())
    }
}

impl<P: Into<SimpleDataPoint<P>> + Clone + fmt::Debug> DataPoint for SimpleDataPoint<P> {
    fn coordinates(&self) -> Vec<f64> {
        self.coordinates.clone()
    }
    fn set_coordinates(&mut self, coordinates: Vec<f64>) {
        self.coordinates = coordinates;
    }
    fn num_dimensions(&self) -> usize {
        self.num_dimensions
    }
}
