// unweighted_graph.rs
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
use chapter4::graph::Graph;
use chapter4::edge::Edge;

pub struct UnweightedGraph<V: Clone + PartialEq> {
    vertices: Vec<V>,
    edges: Vec<Vec<Edge>>,
}

impl<V: Clone + PartialEq> Graph for UnweightedGraph<V> {
    type Vertex = V;
    fn new(vertices: impl Iterator<Item = V>) -> Self {
        let vertices = Vec::from_iter(vertices);
        let edges = vertices.iter().map(|_| Vec::<Edge>::new()).collect();
        UnweightedGraph { vertices, edges }
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
        self.edges.push(Vec::<Edge>::new());
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
            .map(|edge| self.vertex_at(edge.v))
            .collect()
    }
    // Return all of the edges associated with a vertex at some index
    fn edges_of_index(&self, index: usize) -> Vec<Edge> {
        self.edges[index].clone()
    }
    // This is an undirected graph, so we always add edges in both directions
    fn add_edge(&mut self, edge: Edge) {
        self.edges[edge.v].push(edge.reversed());
        self.edges[edge.u].push(edge);
    }
}

impl<V: Clone + PartialEq + ToString> ToString for UnweightedGraph<V> {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..self.get_vertex_count() {
            result.push_str(&format!(
                "{} -> {}\n",
                self.vertex_at(i).to_string(),
                vec_to_string(&self.neighbors_of_index(i))
            ));
        }
        result
    }
}

fn vec_to_string<V: ToString>(list: &[V]) -> String {
    let mut result = String::from("[");
    for s in list.iter().map(|v| v.to_string()) {
        result.push_str(&format!("{s}, "));
    }
    result.pop();
    result.pop();
    result.push(']');
    result
}

impl UnweightedGraph<String> {
    pub fn add_edge_by_string(&mut self, first: &str, second: &str) {
        self.add_edge_by_vertices(&first.to_string(), &second.to_string());
    }
}

fn main() {
    let mut city_graph = UnweightedGraph::<String>::new(
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
