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
use crate::edge::Edge;
use crate::edge::SimpleEdge;
use crate::graph::Graph;
use crate::vec_to_string;

pub struct UnweightedGraph<V: Clone + PartialEq> {
    vertices: Vec<V>,
    edges: Vec<Vec<SimpleEdge>>,
}

impl<V: Clone + PartialEq> UnweightedGraph<V> {
    // Add an edge using vertex indices (convenience method)
    // fn add_edge_by_indices(&mut self, u: usize, v: usize) {
    //     self.add_edge(SimpleEdge::new(u, v));
    // }
    // Add an edge by looking up vertex indices (convenience method)
    fn add_edge_by_vertices(
        &mut self,
        first: &<UnweightedGraph<V> as Graph>::Vertex,
        second: &<UnweightedGraph<V> as Graph>::Vertex,
    ) {
        self.add_edge(SimpleEdge::new(self.index_of(first), self.index_of(second)));
    }
}

impl<V: Clone + PartialEq> Graph for UnweightedGraph<V> {
    type Vertex = V;
    type SizedEdge = SimpleEdge;
    fn new(vertices: impl Iterator<Item = V>) -> Self {
        let vertices = Vec::from_iter(vertices);
        let edges = vertices.iter().map(|_| Vec::<SimpleEdge>::new()).collect();
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
        self.edges.push(Vec::<SimpleEdge>::new());
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
    fn edges_of_index(&self, index: usize) -> Vec<SimpleEdge> {
        self.edges[index].clone()
    }
    // This is an undirected graph, so we always add edges in both directions
    fn add_edge(&mut self, edge: SimpleEdge) {
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

// to see the output when testing, use
// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let mut city_graph = UnweightedGraph::<&str>::new(
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

        city_graph.add_edge_by_vertices(&"Seattle", &"Chicago");
        city_graph.add_edge_by_vertices(&"Seattle", &"San Francisco");
        city_graph.add_edge_by_vertices(&"San Francisco", &"Riverside");
        city_graph.add_edge_by_vertices(&"San Francisco", &"Los Angeles");
        city_graph.add_edge_by_vertices(&"Los Angeles", &"Riverside");
        city_graph.add_edge_by_vertices(&"Los Angeles", &"Phoenix");
        city_graph.add_edge_by_vertices(&"Riverside", &"Phoenix");
        city_graph.add_edge_by_vertices(&"Riverside", &"Chicago");
        city_graph.add_edge_by_vertices(&"Phoenix", &"Dallas");
        city_graph.add_edge_by_vertices(&"Phoenix", &"Houston");
        city_graph.add_edge_by_vertices(&"Dallas", &"Chicago");
        city_graph.add_edge_by_vertices(&"Dallas", &"Atlanta");
        city_graph.add_edge_by_vertices(&"Dallas", &"Houston");
        city_graph.add_edge_by_vertices(&"Houston", &"Atlanta");
        city_graph.add_edge_by_vertices(&"Houston", &"Miami");
        city_graph.add_edge_by_vertices(&"Atlanta", &"Chicago");
        city_graph.add_edge_by_vertices(&"Atlanta", &"Washington");
        city_graph.add_edge_by_vertices(&"Atlanta", &"Miami");
        city_graph.add_edge_by_vertices(&"Miami", &"Washington");
        city_graph.add_edge_by_vertices(&"Chicago", &"Detroit");
        city_graph.add_edge_by_vertices(&"Detroit", &"Boston");
        city_graph.add_edge_by_vertices(&"Detroit", &"Washington");
        city_graph.add_edge_by_vertices(&"Detroit", &"New York");
        city_graph.add_edge_by_vertices(&"Boston", &"New York");
        city_graph.add_edge_by_vertices(&"New York", &"Philadelphia");
        city_graph.add_edge_by_vertices(&"Philadelphia", &"Washington");

        println!("{}", city_graph.to_string());

        let bfs_result =
            generic_search::bfs("Boston", |v| *v == "Miami", |v| city_graph.neighbors_of(v));

        match bfs_result {
            None => println!("No solution found using breadth-first search"),
            Some(node) => {
                let path = generic_search::node_to_path(&node);
                println!("Path from Boston to Miami: {}", vec_to_string(&path));
            }
        }
    }
}
