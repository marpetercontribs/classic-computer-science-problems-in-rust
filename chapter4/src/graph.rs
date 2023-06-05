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

pub struct Graph<V: Clone + PartialEq> {
    vertices: Vec<V>,
    edges: Vec<Vec<Edge>>,
}

impl<V: Clone + PartialEq> Graph<V> {
    pub fn new(vertices: impl Iterator<Item = V>) -> Self {
        let vertices = Vec::from_iter(vertices);
        let edges = vertices.iter().map(|_| Vec::<Edge>::new()).collect();
        Graph { vertices, edges }
    }
    // Number of vertices
    pub fn get_vertex_count(&self) -> usize {
        self.vertices.len()
    }
    // Number of edges
    pub fn get_edge_count(&self) -> usize {
        self.edges.iter().flatten().count()
    }
    // Add a vertex to the graph and return its index
    pub fn add_vertex(&mut self, vertex: V) -> usize {
        self.vertices.push(vertex);
        self.edges.push(Vec::<Edge>::new());
        self.get_vertex_count() - 1
    }
    // Find the vertex at a specific index
    pub fn vertex_at(&self, index: usize) -> V {
        self.vertices[index].clone()
    }
    // Find the index of a vertex in the graph
    pub fn index_of(&self, vertex: &V) -> usize {
        self.vertices.iter().position(|v| v == vertex).unwrap()
    }
    // Find the vertices that a vertex at some index is connected to
    pub fn neighbors_of_index(&self, index: usize) -> Vec<V> {
        self.edges[index]
            .iter()
            .map(|edge| self.vertex_at(edge.v))
            .collect()
    }
    // Look up a vertice's index and find its neighbors (convenience method)
    pub fn neighbors_of(&self, vertex: &V) -> Vec<V> {
        self.neighbors_of_index(self.index_of(vertex))
    }
    // Return all of the edges associated with a vertex at some index
    pub fn edges_of_index(&self, index: usize) -> Vec<Edge> {
        self.edges[index].clone()
    }
    // Look up the index of a vertex and return its edges (convenience method)
    pub fn edges_of(&self, vertex: &V) -> Vec<Edge> {
        self.edges_of_index(self.index_of(&vertex))
    }

    // This is an undirected graph, so we always add
    // edges in both directions
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges[edge.v].push(edge.reversed());
        self.edges[edge.u].push(edge);
    }

    // Add an edge using vertex indices (convenience method)
    pub fn add_edge_by_indices(&mut self, u: usize, v: usize) {
        self.add_edge(Edge::new(u, v));
    }

    // Add an edge by looking up vertex indices (convenience method)
    pub fn add_edge_by_vertices(&mut self, first: &V, second: &V) {
        self.add_edge(Edge::new(self.index_of(first), self.index_of(second)));
    }
}

impl<V: Clone + PartialEq + std::fmt::Display> ToString for Graph<V> {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..self.get_vertex_count() {
            result.push_str(&format!(
                "{} -> {}\n",
                self.vertex_at(i),
                vec_to_string(&self.neighbors_of_index(i))
            ));
        }
        result
    }
}

fn vec_to_string<V: std::fmt::Display>(list: &Vec<V>) -> String {
    let mut result = String::from("[");
    for s in list.iter().map(|v| v.to_string()) {
        result.push_str(&format!("{s}, "));
    }
    result.pop();
    result.pop();
    result.push(']');
    result
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
            .iter().map(|s| s.to_string()),
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