// album.rs
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
use chapter6::data_point::{SimpleDataPoint};
use chapter6::kmeans::KMeans;
use std::fmt;

// Instead of inheriting from [Simple]DataPoint, which Rust does not support, "decorate" SimpleDataPoint
#[derive(Clone)]
struct Album {
    name: String,
    year: usize,
    length: f64,
    tracks: usize,
}

impl fmt::Debug for Album {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({}, {})", self.name, self.year)
    }
}

impl Album {
    fn new(name: &str, year: usize, length: f64, tracks: usize) -> Self {
        Album {
            name: name.to_string(),
            year,
            length,
            tracks,
        }
    }
}

// Implemnting From<Album> for SimpleDataPoint automatically implements
// trait Into<SimpleDataPoint> for Album
impl From<Album> for SimpleDataPoint<Album> {
    fn from(item: Album) -> Self {
        Self {
            num_dimensions: 2,
            coordinates: vec![item.length, item.tracks as f64],
            original: item,
        }
    }
}

fn main() {
    let albums = vec![
        Album::new("Got to Be There", 1972, 35.45, 10),
        Album::new("Ben", 1972, 31.31, 10),
        Album::new("Music & Me", 1973, 32.09, 10),
        Album::new("Forever, Michael", 1975, 33.36, 10),
        Album::new("Off the Wall", 1979, 42.28, 10),
        Album::new("Thriller", 1982, 42.19, 9),
        Album::new("Bad", 1987, 48.16, 10),
        Album::new("Dangerous", 1991, 77.03, 14),
        Album::new(
            "HIStory: Past, Present and Future, Book I",
            1995,
            148.58,
            30,
        ),
        Album::new("Invincible", 2001, 77.05, 16),
    ];
    let mut kmeans: KMeans<Album> = KMeans::new(2, albums);
    let clusters = kmeans.run(100);
    for (cluster_no, cluster) in clusters.iter().enumerate() {
        println!("Cluster {cluster_no}: {:?}", cluster.points);
    }
}
