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
    type SizedEdge: Edge + Sized;
    fn new(vertices: impl Iterator<Item = Self::Vertex>) -> Self;
    // Number of vertices
    fn get_vertex_count(&self) -> usize;
    // Number of edges
    fn get_edge_count(&self) -> usize;
    // Add a vertex to the graph and return its index
    fn add_vertex(&mut self, vertex: Self::Vertex) -> usize;
    // Find the vertex at a specific index
    fn index_of(&self, vertex: &Self::Vertex) -> usize;
    // Find the index of a vertex in the graph
    fn vertex_at(&self, index: usize) -> Self::Vertex;
    // Find the vertices that a vertex at some index is connected to
    fn neighbors_of_index(&self, index: usize) -> Vec<Self::Vertex>;
    // Return all of the edges associated with a vertex at some index
    fn edges_of_index(&self, index: usize) -> Vec<Self::SizedEdge>;
    // This is an undirected graph, so we always add edges in both directions
    fn add_edge(&mut self, edge: Self::SizedEdge);

    // Look up a vertice's index and find its neighbors (convenience method)
    fn neighbors_of(&self, vertex: &Self::Vertex) -> Vec<Self::Vertex> {
        self.neighbors_of_index(self.index_of(vertex))
    }
    // Look up the index of a vertex and return its edges (convenience method)
    fn edges_of(&self, vertex: &Self::Vertex) -> Vec<Self::SizedEdge> {
        self.edges_of_index(self.index_of(vertex))
    }
}
