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
use crate::statistics::Statistics;
use std::fmt;
use std::rc::Rc;

use rand::{thread_rng, Rng};

// Rust does not support inheritance for structs, and traits cannot contain data.
// Thus we cannot make the things to cluster inherit from a DataPoint struct
// But all we need for the cluster coordinates is the ability to convert the things
// to cluster into DataPoints
// => Use a concrete implementation of DataPoint, not just a trait
//    base Cluster and KMeans on things that implement Into<DataPoint>

#[derive(Clone)]
pub struct Cluster<P: Into<DataPoint<P>> + Clone + fmt::Debug> {
    pub points: Vec<Rc<DataPoint<P>>>, // to avoid copying the DataPoints for each cluster, use shareable references
    centroid: DataPoint<Vec<f64>>,
}
impl<P: Into<DataPoint<P>> + Clone + fmt::Debug> Cluster<P> {
    fn new(points: &[Rc<DataPoint<P>>], centroid: DataPoint<Vec<f64>>) -> Self {
        Cluster {
            points: points.to_vec(),
            centroid,
        }
    }
}

pub struct KMeans<P: Into<DataPoint<P>> + Clone + fmt::Debug> {
    points: Vec<Rc<DataPoint<P>>>, // to avoid copying the DataPoints for each cluster, shareable references must be used by KMeans as well
    clusters: Vec<Cluster<P>>,
}
impl<P: Into<DataPoint<P>> + Clone + fmt::Debug> KMeans<P> {
    pub fn new(k: usize, points: Vec<P>) -> Self {
        let z_scored_points = Self::z_score_normalize(points);
        let points: Vec<Rc<DataPoint<P>>> =
            z_scored_points.clone().into_iter().map(Rc::new).collect();
        let mut clusters = Vec::<Cluster<P>>::with_capacity(k);
        (0..k).for_each(|_| {
            let cluster = Cluster::<P>::new(&points, Self::random_point(&z_scored_points));
            clusters.push(cluster);
        });
        KMeans { points, clusters }
    }
    pub fn run(&mut self, max_iterations: usize) -> Vec<Cluster<P>> {
        for iteration in 0..max_iterations {
            for cluster in self.clusters.iter_mut() {
                cluster.points.clear();
            }
            self.assign_clusters();
            let old_centroids = self.centroids();
            self.generate_centroids();
            if Self::coordinates_are_equal(&old_centroids, &self.centroids()) {
                println!("Converged after {iteration} iterations.");
                return self.clusters.clone();
            }
        }
        self.clusters.clone()
    }

    fn z_score_normalize(points: Vec<P>) -> Vec<DataPoint<P>> {
        // Convert the things to cluster into DataPoints
        let points: Vec<DataPoint<P>> = points.into_iter().map(|p| p.into()).collect();
        let num_dimensions = points[0].num_dimensions();
        let mut z_scored_points = vec![Vec::<f64>::with_capacity(num_dimensions); points.len()];

        for dimension in 0..num_dimensions {
            let dimension_values = Self::dimension_slice(&points, dimension);
            let zscored_values = Statistics::zscore(&dimension_values);
            for (index, zscore) in zscored_values.iter().enumerate() {
                z_scored_points[index].push(*zscore);
            }
        }
        let mut points = points.to_vec();
        for (index, point) in points.iter_mut().enumerate() {
            point.set_coordinates(z_scored_points[index].clone());
        }
        points
    }
    fn centroids(&self) -> Vec<DataPoint<Vec<f64>>> {
        self.clusters
            .iter()
            .map(|cluster| cluster.centroid.clone())
            .collect()
    }
    fn assign_clusters(&mut self) {
        for point in self.points.iter() {
            let mut lowest_distance = f64::MAX;
            let mut closest_cluster_index = 0;
            for (cluster_index, cluster) in self.clusters.iter().enumerate() {
                let centroid_distance = point.distance(&cluster.centroid);
                if centroid_distance < lowest_distance {
                    lowest_distance = centroid_distance;
                    closest_cluster_index = cluster_index;
                }
            }
            self.clusters[closest_cluster_index]
                .points
                .push(point.clone());
        }
    }
    fn generate_centroids(&mut self) {
        for cluster in self.clusters.iter_mut() {
            if !cluster.points.is_empty() {
                let mut means = Vec::<f64>::new();
                let num_dimensions = cluster.points[0].num_dimensions();
                for dimension in 0..num_dimensions {
                    let dimension_mean: f64 = cluster
                        .points
                        .iter()
                        .map(|point| point.coordinates()[dimension])
                        .sum::<f64>()
                        / (num_dimensions as f64);
                    means.push(dimension_mean);
                }
                cluster.centroid = DataPoint::from(means);
            }
        }
    }
    fn coordinates_are_equal(this: &[DataPoint<Vec<f64>>], that: &[DataPoint<Vec<f64>>]) -> bool {
        if this.len() != that.len() {
            return false;
        } else {
            for (index, this_point) in this.iter().enumerate() {
                let that_coordinates = that[index].coordinates();
                for (dimension, coordinate) in this_point.coordinates().iter().enumerate() {
                    if *coordinate != that_coordinates[dimension] {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn random_point(points: &[DataPoint<P>]) -> DataPoint<Vec<f64>> {
        let mut rng = thread_rng();
        let mut initials = Vec::<f64>::new();
        for dimension in 0..points[0].num_dimensions() {
            let dimension_values = Self::dimension_slice(points, dimension);
            let stats = Statistics::new(&dimension_values);
            let random_value = rng.gen_range(stats.min..stats.max);
            initials.push(random_value);
        }
        DataPoint::from(initials)
    }
    fn dimension_slice(points: &[DataPoint<P>], dimension: usize) -> Vec<f64> {
        points
            .iter()
            .map(|point| point.coordinates()[dimension])
            .collect()
    }
}

// to see the output when testing, use
// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let point_1 = vec![2.0, 1.0, 1.0];
        let point_2 = vec![2.0, 2.0, 5.0];
        let point_3 = vec![3.0, 1.5, 2.5];
        let mut kmeans_test: KMeans<Vec<f64>> = KMeans::new(2, vec![point_1, point_2, point_3]);
        let test_clusters = kmeans_test.run(100);
        for (cluster_no, cluster) in test_clusters.iter().enumerate() {
            println!("Cluster {cluster_no}: {:?}", cluster.points);
        }
    }
}
