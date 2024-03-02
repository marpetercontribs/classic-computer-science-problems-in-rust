// governor.rs
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
use chapter6::data_point::DataPoint;
use chapter6::kmeans::KMeans;
use std::fmt;

#[derive(Clone)]
struct Governor {
    original_longitude: f64,
    original_age: f64,
    longitude: f64,
    age: f64,
    state: String,
}

impl fmt::Debug for Governor {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}: (longitude: {}, age: {})",
            self.state, self.original_longitude, self.original_age
        )
    }
}

impl Governor {
    fn new(longitude: f64, age: usize, state: &str) -> Self {
        Governor {
            original_longitude: longitude,
            original_age: age as f64,
            longitude,
            age: age as f64,
            state: state.to_string(),
        }
    }
}

impl DataPoint for Governor {
    fn originals(&self) -> Vec<f64> {
        vec![self.original_longitude, self.original_age]
    }
    fn coordinates(&self) -> Vec<f64> {
        vec![self.longitude, self.age]
    }
    fn set_coordinates(&mut self, coordinates: Vec<f64>) {
        self.longitude = coordinates[0];
        self.age = coordinates[1];
    }
    fn num_dimensions(&self) -> usize {
        2
    }
}

fn main() {
    let governors = vec![
        Governor::new(-86.79113, 72, "Alabama"),
        Governor::new(-152.404419, 66, "Alaska"),
        Governor::new(-111.431221, 53, "Arizona"),
        Governor::new(-92.373123, 66, "Arkansas"),
        Governor::new(-119.681564, 79, "California"),
        Governor::new(-105.311104, 65, "Colorado"),
        Governor::new(-72.755371, 61, "Connecticut"),
        Governor::new(-75.507141, 61, "Delaware"),
        Governor::new(-81.686783, 64, "Florida"),
        Governor::new(-83.643074, 74, "Georgia"),
        Governor::new(-157.498337, 60, "Hawaii"),
        Governor::new(-114.478828, 75, "Idaho"),
        Governor::new(-88.986137, 60, "Illinois"),
        Governor::new(-86.258278, 49, "Indiana"),
        Governor::new(-93.210526, 57, "Iowa"),
        Governor::new(-96.726486, 60, "Kansas"),
        Governor::new(-84.670067, 50, "Kentucky"),
        Governor::new(-91.867805, 50, "Louisiana"),
        Governor::new(-69.381927, 68, "Maine"),
        Governor::new(-76.802101, 61, "Maryland"),
        Governor::new(-71.530106, 60, "Massachusetts"),
        Governor::new(-84.536095, 58, "Michigan"),
        Governor::new(-93.900192, 70, "Minnesota"),
        Governor::new(-89.678696, 62, "Mississippi"),
        Governor::new(-92.288368, 43, "Missouri"),
        Governor::new(-110.454353, 51, "Montana"),
        Governor::new(-98.268082, 52, "Nebraska"),
        Governor::new(-117.055374, 53, "Nevada"),
        Governor::new(-71.563896, 42, "New Hampshire"),
        Governor::new(-74.521011, 54, "New Jersey"),
        Governor::new(-106.248482, 57, "New Mexico"),
        Governor::new(-74.948051, 59, "New York"),
        Governor::new(-79.806419, 60, "North Carolina"),
        Governor::new(-99.784012, 60, "North Dakota"),
        Governor::new(-82.764915, 65, "Ohio"),
        Governor::new(-96.928917, 62, "Oklahoma"),
        Governor::new(-122.070938, 56, "Oregon"),
        Governor::new(-77.209755, 68, "Pennsylvania"),
        Governor::new(-71.51178, 46, "Rhode Island"),
        Governor::new(-80.945007, 70, "South Carolina"),
        Governor::new(-99.438828, 64, "South Dakota"),
        Governor::new(-86.692345, 58, "Tennessee"),
        Governor::new(-97.563461, 59, "Texas"),
        Governor::new(-111.862434, 70, "Utah"),
        Governor::new(-72.710686, 58, "Vermont"),
        Governor::new(-78.169968, 60, "Virginia"),
        Governor::new(-121.490494, 66, "Washington"),
        Governor::new(-80.954453, 66, "West Virginia"),
        Governor::new(-89.616508, 49, "Wisconsin"),
        Governor::new(-107.30249, 55, "Wyoming"),
    ];

    let mut kmeans: KMeans<Governor> = KMeans::new(2, governors);
    let clusters = kmeans.run(100);
    for (cluster_no, cluster) in clusters.iter().enumerate() {
        println!("Cluster {cluster_no}: {:?}", cluster.points);
    }
}
