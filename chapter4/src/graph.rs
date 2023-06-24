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
    // Exercise 4.1: Remove a vertex from the graph
    fn remove_vertex(&mut self, vertex: Self::Vertex);
    // Add an edge to the graph; for undirected graphs add edges in both directions
    fn add_edge(&mut self, edge: Self::SizedEdge);
    // Exercise 4.1: This is an undirected graph, so we always remove edges in both directions
    fn remove_edge(&mut self, edge: &Self::SizedEdge);
    // Exercise 4.1: Remove an edge by looking up vertex indices (convenience method)
    fn remove_edge_by_vertices(&mut self, first: &Self::Vertex, second: &Self::Vertex) {
        let u = self.index_of(first);
        let v = self.index_of(second);
        let edges = self.edges();
        let edge = edges[u].iter().find(|e| e.v() == v).unwrap();
        self.remove_edge(&edge);
    }
    // Number of vertices
    fn get_vertex_count(&self) -> usize {
        self.vertices().len()
    }
    // Number of edges
    fn get_edge_count(&self) -> usize {
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
