// graph.rs
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

pub trait Graph {
    type Vertex: Clone + PartialEq;
    type SizedEdge: Edge + Sized + Clone;
    fn new(vertices: impl Iterator<Item = Self::Vertex>) -> Self;
    // "getter" methods to allow implementing most methods on trait level
    fn vertices(&self) -> Vec<Self::Vertex>;
    fn edges(&self) -> Vec<Vec<Self::SizedEdge>>;
    // Add a vertex to the graph and return its index
    fn add_vertex(&mut self, vertex: Self::Vertex) -> usize;
    // Add an edge to the graph; for undirected graphs add edges in both directions
    fn add_edge(&mut self, edge: Self::SizedEdge);
    // Find the vertex at a specific index    // Number of vertices
    fn get_vertex_count(&self) -> usize {
        self.vertices().len()
    }
    // Number of edges
    fn get_edge_count(&self) -> usize{
        self.edges().iter().flatten().count()
    }
   // Find the index of a vertex in the graph
    fn index_of(&self, vertex: &Self::Vertex) -> usize {
        self.vertices().iter().position(|v| v == vertex).unwrap()
    }
    // Find the vertex at a specific index
    fn vertex_at(&self, index: usize) -> Self::Vertex {
        self.vertices()[index].clone()
    }
    // Find the vertices that a vertex at some index is connected to
    fn neighbors_of_index(&self, index: usize) -> Vec<Self::Vertex> {
        self.edges()[index]
            .iter()
            .map(|edge| self.vertex_at(edge.v()))
            .collect()
    }
    // Return all of the edges associated with a vertex at some index
    fn edges_of_index(&self, index: usize) -> Vec<Self::SizedEdge> {
        self.edges()[index].clone()
    }
    // Look up a vertice's index and find its neighbors (convenience method)
    fn neighbors_of(&self, vertex: &Self::Vertex) -> Vec<Self::Vertex> {
        self.neighbors_of_index(self.index_of(vertex))
    }
    // Look up the index of a vertex and return its edges (convenience method)
    fn edges_of(&self, vertex: &Self::Vertex) -> Vec<Self::SizedEdge> {
        self.edges_of_index(self.index_of(vertex))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Graph<String> {
        fn add_edge_by_string(&mut self, first: &str, second: &str) {
            self.add_edge_by_vertices(&first.to_string(), &second.to_string());
        }
    }

    #[test]
    fn basic_graph_construction_works() {
        let mut city_graph = Graph::<String>::new(
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
            .iter()
            .map(|s| s.to_string()),
        );

        city_graph.add_edge_by_string("Seattle", "Chicago");
        city_graph.add_edge_by_string("Seattle", "San Francisco");
        city_graph.add_edge_by_string("San Francisco", "Riverside");
        city_graph.add_edge_by_string("San Francisco", "Los Angeles");
        city_graph.add_edge_by_string("Los Angeles", "Riverside");
        city_graph.add_edge_by_string("Los Angeles", "Phoenix");
        city_graph.add_edge_by_string("Riverside", "Phoenix");
        city_graph.add_edge_by_string("Riverside", "Chicago");
        city_graph.add_edge_by_string("Phoenix", "Dallas");
        city_graph.add_edge_by_string("Phoenix", "Houston");
        city_graph.add_edge_by_string("Dallas", "Chicago");
        city_graph.add_edge_by_string("Dallas", "Atlanta");
        city_graph.add_edge_by_string("Dallas", "Houston");
        city_graph.add_edge_by_string("Houston", "Atlanta");
        city_graph.add_edge_by_string("Houston", "Miami");
        city_graph.add_edge_by_string("Atlanta", "Chicago");
        city_graph.add_edge_by_string("Atlanta", "Washington");
        city_graph.add_edge_by_string("Atlanta", "Miami");
        city_graph.add_edge_by_string("Miami", "Washington");
        city_graph.add_edge_by_string("Chicago", "Detroit");
        city_graph.add_edge_by_string("Detroit", "Boston");
        city_graph.add_edge_by_string("Detroit", "Washington");
        city_graph.add_edge_by_string("Detroit", "New York");
        city_graph.add_edge_by_string("Boston", "New York");
        city_graph.add_edge_by_string("New York", "Philadelphia");
        city_graph.add_edge_by_string("Philadelphia", "Washington");

        println!("{}", city_graph.to_string());
    }
}
