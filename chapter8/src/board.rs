// board.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 8
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

pub trait Piece {
    fn opposite(&self) -> Self
    where
        Self: Sized;
}

pub trait Board<P: Piece, M: Copy> {
    fn turn(&self) -> P;
    fn do_move(&self, location: M) -> Self
    where
        Self: Sized; // move not allowed as method name
    fn legal_moves(&self) -> Vec<M>;
    fn is_win(&self) -> bool;
    fn is_draw(&self) -> bool {
        !self.is_win() && self.legal_moves().is_empty()
    }
    fn evaluate(&self, player: &P) -> f64;
}
