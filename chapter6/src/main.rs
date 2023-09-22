// main.rs
// Adapted for Rust version
// of Classic Computer Science Problems in Python/Java Chapter 6
//
// This code does not directly correspond to any of the problems covered in
// the books, but to have a file structure similar to Kopec's (on github)
// compatible with rust's package manager cargo, with one folder per chapter
// without having to change the generated Cargo.toml for each program
//   of chapter 6 in the books, and
// without having a seperate rust package and sub-folder for each program
//   with its own src folder and generic file name main.rs
//
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

fn main() {
    println!(
        "Use 'cargo test <program name> -- --nocapture' to run kmeans belonging to this chapter."
    );
    println!("Use 'cargo run --bin <program name>' to run one of the other programs belonging to this chapter.");
}
