// dijkstra.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 4
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

use chapter4::edge::WeightedEdge;
use chapter4::graph::Graph;
use chapter4::weighted_graph::WeightedGraph;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq)]
struct DijkstraNode {
    vertex: usize,
    distance: f64,
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff = self.distance - other.distance;
        if diff < -0.000001_f64 {
            Ordering::Greater // Note the reversed order, because Rust's BinaryHeap pops highest priority element first!
        } else if diff > 0.000001_f64 {
            Ordering::Less // Note the reversed order, because Rust's BinaryHeap pops highest priority element first!
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for DijkstraNode {}

fn dijkstra<V: Clone + PartialEq>(
    wg: &WeightedGraph<V>,
    start: &V,
) -> (Vec<Option<f64>>, HashMap<usize, WeightedEdge>) {
    let first = wg.index_of(start); // find starting index
    let mut distances: Vec<Option<f64>> = vec![None; wg.get_vertex_count()]; // distances are unknown at first
    distances[first] = Some(0_f64); // the root is 0 away from the root
    let mut path_map = HashMap::<usize, WeightedEdge>::new(); // how we got to each vertex
    let mut pq = BinaryHeap::<DijkstraNode>::new();
    pq.push(DijkstraNode {
        vertex: first,
        distance: 0_f64,
    });

    while let Some(node) = pq.pop() {
        let u = node.vertex; // explore the next closest vertex
        let dist_u = distances[u].unwrap(); // should already have seen it
        for we in wg.edges_of_index(u) {
            // look at every edge/vertex from the vertex in question
            let dist_v = distances[we.simple_edge.v]; // the old distance to this vertex
            if dist_v == None || dist_v.unwrap() > we.weight + dist_u {
                // no old distance or found shorter path
                distances[we.simple_edge.v] = Some(we.weight + dist_u); // update distance to this vertex
                path_map.insert(we.simple_edge.v, we.clone()); // update the edge on the shortest path to this vertex
                pq.push(DijkstraNode {
                    // explore it soon
                    vertex: we.simple_edge.v,
                    distance: we.weight + dist_u,
                });
            }
        }
    }
    (distances, path_map)
}


fn distance_array_to_vertex_dict<V: PartialEq + Clone + Eq + Hash>(wg: &WeightedGraph<V>, distances: Vec<Option<f64>>) -> HashMap<V,Option<f64>> {
    let mut result = HashMap::<V,Option<f64>>::new();
    for i in 0..distances.len() {
        result.insert(wg.vertex_at(i), distances[i]);
    }
    result
}

fn path_map_to_path(start: usize, end: usize, path_map: HashMap<usize,WeightedEdge>) -> Vec<WeightedEdge> {
    let mut result = Vec::<WeightedEdge>::new();
    if path_map.len() > 0 {
        let mut edge = path_map.get(&end).unwrap();
        result.push(edge.clone());
        while edge.simple_edge.u != start {
            edge = path_map.get(&edge.simple_edge.u).unwrap();
            result.push(edge.clone());
        }
    }
    result.reverse();
    result
}

fn main() {
    let mut city_graph = WeightedGraph::<&str>::new(
        [
            "Seattle",
            "San Francisco",
            "Los Angeles",
            "Riverside",
            "Phoenix",
            "Chicago",
            "Boston",
            "New York",
            "Atlanta",
            "Miami",
            "Dallas",
            "Houston",
            "Detroit",
            "Philadelphia",
            "Washington",
        ]
        .into_iter(),
    );

    city_graph.add_edge_by_vertices(&"Seattle", &"Chicago", 1737.0);
    city_graph.add_edge_by_vertices(&"Seattle", &"San Francisco", 678.0);
    city_graph.add_edge_by_vertices(&"San Francisco", &"Riverside", 386.0);
    city_graph.add_edge_by_vertices(&"San Francisco", &"Los Angeles", 348.0);
    city_graph.add_edge_by_vertices(&"Los Angeles", &"Riverside", 50.0);
    city_graph.add_edge_by_vertices(&"Los Angeles", &"Phoenix", 357.0);
    city_graph.add_edge_by_vertices(&"Riverside", &"Phoenix", 307.0);
    city_graph.add_edge_by_vertices(&"Riverside", &"Chicago", 1704.0);
    city_graph.add_edge_by_vertices(&"Phoenix", &"Dallas", 887.0);
    city_graph.add_edge_by_vertices(&"Phoenix", &"Houston", 1015.0);
    city_graph.add_edge_by_vertices(&"Dallas", &"Chicago", 805.0);
    city_graph.add_edge_by_vertices(&"Dallas", &"Atlanta", 721.0);
    city_graph.add_edge_by_vertices(&"Dallas", &"Houston", 225.0);
    city_graph.add_edge_by_vertices(&"Houston", &"Atlanta", 702.0);
    city_graph.add_edge_by_vertices(&"Houston", &"Miami", 968.0);
    city_graph.add_edge_by_vertices(&"Atlanta", &"Chicago", 588.0);
    city_graph.add_edge_by_vertices(&"Atlanta", &"Washington", 543.0);
    city_graph.add_edge_by_vertices(&"Atlanta", &"Miami", 604.0);
    city_graph.add_edge_by_vertices(&"Miami", &"Washington", 923.0);
    city_graph.add_edge_by_vertices(&"Chicago", &"Detroit", 238.0);
    city_graph.add_edge_by_vertices(&"Detroit", &"Boston", 613.0);
    city_graph.add_edge_by_vertices(&"Detroit", &"Washington", 396.0);
    city_graph.add_edge_by_vertices(&"Detroit", &"New York", 482.0);
    city_graph.add_edge_by_vertices(&"Boston", &"New York", 190.0);
    city_graph.add_edge_by_vertices(&"New York", &"Philadelphia", 81.0);
    city_graph.add_edge_by_vertices(&"Philadelphia", &"Washington", 123.0);

    println!("{}", city_graph.to_string());

    println!("");
    let (distances, path_map) = dijkstra(&city_graph, &"Los Angeles");
    let name_distance = distance_array_to_vertex_dict(&city_graph,distances);
    println!("Distances from Los Angeles:");
    for (key, value) in name_distance.iter() {
        let real_value = value.unwrap();
        println!("{key}: {real_value}");
    }

    println!("");
    println!("Shortest path from Los Angeles to Boston:");
    let path = path_map_to_path(city_graph.index_of(&"Los Angeles"), city_graph.index_of(&"Boston"), path_map);
    println!("{}", city_graph.path_to_string(&path));
}
