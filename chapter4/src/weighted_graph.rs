// weighted_graph.rs
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
use crate::edge::Edge;
use crate::edge::WeightedEdge;
use crate::graph::Graph;
use crate::tuple_vec_to_string;

pub struct WeightedGraph<V: Clone + PartialEq> {
    vertices: Vec<V>,
    edges: Vec<Vec<WeightedEdge>>,
}

impl<V: Clone + PartialEq> WeightedGraph<V> {
    // Add an edge by looking up vertex indices (convenience method)
    fn add_edge_by_vertices(
        &mut self,
        first: &<WeightedGraph<V> as Graph>::Vertex,
        second: &<WeightedGraph<V> as Graph>::Vertex,
        weight: f64,
    ) {
        self.add_edge(WeightedEdge::new(
            self.index_of(first),
            self.index_of(second),
            weight,
        ));
    }
    // Find the vertices that a vertex at some index is connected to
    fn neighbors_of_index_with_weight(&self, index: usize) -> Vec<(V, f64)> {
        self.edges[index]
            .iter()
            .map(|edge| (self.vertex_at(edge.simple_edge.v), edge.weight))
            .collect()
    }
}

impl<V: Clone + PartialEq> Graph for WeightedGraph<V> {
    type Vertex = V;
    type SizedEdge = WeightedEdge;
    fn new(vertices: impl Iterator<Item = V>) -> Self {
        let vertices = Vec::from_iter(vertices);
        let edges = vertices
            .iter()
            .map(|_| Vec::<WeightedEdge>::new())
            .collect();
        WeightedGraph { vertices, edges }
    }
    // Number of vertices
    fn get_vertex_count(&self) -> usize {
        self.vertices.len()
    }
    // Number of edges
    fn get_edge_count(&self) -> usize {
        self.edges.iter().flatten().count()
    }
    // Add a vertex to the graph and return its index
    fn add_vertex(&mut self, vertex: V) -> usize {
        self.vertices.push(vertex);
        self.edges.push(Vec::<WeightedEdge>::new());
        self.get_vertex_count() - 1
    }
    // Find the vertex at a specific index
    fn vertex_at(&self, index: usize) -> V {
        self.vertices[index].clone()
    }
    // Find the index of a vertex in the graph
    fn index_of(&self, vertex: &V) -> usize {
        self.vertices.iter().position(|v| v == vertex).unwrap()
    }
    // Find the vertices that a vertex at some index is connected to
    fn neighbors_of_index(&self, index: usize) -> Vec<V> {
        self.edges[index]
            .iter()
            .map(|edge| self.vertex_at(edge.simple_edge.v))
            .collect()
    }
    // Return all of the edges associated with a vertex at some index
    fn edges_of_index(&self, index: usize) -> Vec<WeightedEdge> {
        self.edges[index].clone()
    }
    // This is an undirected graph, so we always add edges in both directions
    fn add_edge(&mut self, edge: WeightedEdge) {
        self.edges[edge.simple_edge.v].push(edge.reversed());
        self.edges[edge.simple_edge.u].push(edge);
    }
}

impl<V: Clone + PartialEq + ToString> ToString for WeightedGraph<V> {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..self.get_vertex_count() {
            result.push_str(&format!(
                "{} -> {}\n",
                self.vertex_at(i).to_string(),
                tuple_vec_to_string(&self.neighbors_of_index_with_weight(i))
            ));
        }
        result
    }
}

// to see the output when testing, use
// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
}
