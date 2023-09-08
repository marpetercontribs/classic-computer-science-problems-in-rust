// kmeans.rs
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

use crate::data_point::DataPoint;
use crate::data_point::SimpleDataPoint;
use rand::{thread_rng,Rng};

struct Cluster<P: DataPoint> {
    points: Vec<P>,
    centroid: SimpleDataPoint,
}
impl<P: DataPoint> Cluster<P> {
    fn new(points: &Vec<P>, centroid: SimpleDataPoint) -> Self {
        Cluster {
            points: points.clone(),
            centroid,
        }
    }
}

pub struct KMeans<P: DataPoint> {
    points: Vec<P>,
    clusters: Vec<Cluster<P>>,
}
impl<P: DataPoint> KMeans<P> {
    pub fn new(k: usize, points: &Vec<P>) -> Self {
        let mut clusters = Vec::<Cluster<P>>::new();
        let mut instance = KMeans {
            points: points.clone(),
            clusters,
        };
        for index in 0..k {
            let cluster = Cluster::<P>::new(points, instance.randomPoint());
        }
        instance
    }

    fn randomPoint(&self) -> SimpleDataPoint {
        let mut rng = thread_rng();
        let mut initials = Vec::<f64>::new();
        for dimension in 0..self.points[0].num_dimensions() {
            let dimension_values = self.dimension_slice(dimension);
            let min = dimension_values.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap().clone();
            let max = dimension_values.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap().clone();
            let random_value = rng.gen_range(min..max);
            initials.push(random_value);
        }
        SimpleDataPoint::new(initials)
    }
    fn dimension_slice(&self, dimension: usize) -> Vec<f64> {
        self.points.iter().map(|point| point.coordinates()[dimension]).collect()
    }

}

// to see the output when testing, use
// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let point_1: SimpleDataPoint = SimpleDataPoint::new(vec![2.0, 1.0, 1.0]);
        let point_2: SimpleDataPoint = SimpleDataPoint::new(vec![2.0, 2.0, 5.0]);
        let point_3: SimpleDataPoint = SimpleDataPoint::new(vec![3.0, 1.5, 2.5]);
    }
}
