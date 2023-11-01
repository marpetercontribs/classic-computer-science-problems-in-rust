// tsp.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 9
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
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let mut vt_distances: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    vt_distances.insert(
        "Rutland",
        HashMap::from([
            ("Burlington", 67),
            ("White River Junction", 46),
            ("Bennington", 55),
            ("Brattleboro", 75),
        ]),
    );
    vt_distances.insert(
        "Burlington",
        HashMap::from([
            ("Rutland", 67),
            ("White River Junction", 91),
            ("Bennington", 122),
            ("Brattleboro", 153),
        ]),
    );
    vt_distances.insert(
        "White River Junction",
        HashMap::from([
            ("Rutland", 46),
            ("Burlington", 91),
            ("Bennington", 98),
            ("Brattleboro", 65),
        ]),
    );
    vt_distances.insert(
        "Bennington",
        HashMap::from([
            ("Rutland", 55),
            ("Burlington", 122),
            ("White River Junction", 98),
            ("Brattleboro", 40),
        ]),
    );
    vt_distances.insert(
        "Brattleboro",
        HashMap::from([
            ("Rutland", 75),
            ("Burlington", 153),
            ("White River Junction", 65),
            ("Bennington", 40),
        ]),
    );

    let vt_cities = vt_distances.keys();
    let length = vt_cities.len();
    // let city_permutations = vt_cities.permutations(length);
    // let mut tsp_paths: Vec<Vec<&&str>> = Vec::new();
    // for permutation in city_permutations {
    //     let mut path = permutation.to_vec();
    //     path.push(permutation[0]);
    //     tsp_paths.push( path );
    // }
    let mut tsp_paths: Vec<Vec<&&str>> = vt_cities.permutations(length).collect();
    for path in tsp_paths.iter_mut() {
        path.push(path[0]);
    }

    let mut best_path: Vec<&&str> = Vec::new();
    let mut min_distance: usize = usize::MAX;

    for path in tsp_paths.iter() {
        let mut distance: usize = 0;
        let mut last: &str = path[0];
        for next in path.iter().skip(1) {
            distance += vt_distances.get(last).unwrap().get(*next).unwrap();
            last = next;
        }
        if min_distance > distance {
            min_distance = distance;
            best_path = path.to_vec();
        }
    }

    println!(
        "The shortest path is {:?} in {min_distance} miles.",
        best_path
    );
}
