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

use std::collections::HashMap;

struct DijkstraNode {
    vertex: usize,
    distances: Vec<f64>,
}

fn dijkstra<V: Clone + PartialEq>(wg: &WeightedGraph<V>, start: V ) -> (Vec<Option<f64>>,HashMap<usize,WeightedEdge>) {
    (Vec::<Option<f64>>::new(),HashMap::<usize,WeightedEdge>::new())
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

}