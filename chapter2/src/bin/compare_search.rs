// compare_search.rs
// Solution to exercise 1
// in Classic Computer Science Problems in Python/Java Chapter 2
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

use std::time::Instant;

fn main() {
    let numbers: Vec<usize> = (0..1000000).collect();

    let now = Instant::now();
    println!(
        "linear search for 500 in the list: {} - took {}ns",
        generic_search::linear_contains(&numbers, &500),
        now.elapsed().as_nanos()
    );
    let now = Instant::now();
    println!(
        "binary search for 500 in the list: {} - took {}ns",
        generic_search::binary_contains(&numbers, &500),
        now.elapsed().as_nanos()
    );
    let now = Instant::now();
    println!(
        "linear search for 33300 in the list: {} - took {}ns",
        generic_search::linear_contains(&numbers, &33300),
        now.elapsed().as_nanos()
    );
    let now = Instant::now();
    println!(
        "binary search for 33300 in the list: {} - took {}ns",
        generic_search::binary_contains(&numbers, &33300),
        now.elapsed().as_nanos()
    );
    let now = Instant::now();
    println!(
        "linear search for 75321 in the list: {} - took {}ns",
        generic_search::linear_contains(&numbers, &75321),
        now.elapsed().as_nanos()
    );
    let now = Instant::now();
    println!(
        "binary search for 75321 in the list: {} - took {}ns",
        generic_search::binary_contains(&numbers, &75321),
        now.elapsed().as_nanos()
    );
    let now = Instant::now();
    let third_last = &numbers[999997];
    println!(
        "linear search for {third_last} in the list: {} - took {}ns",
        generic_search::linear_contains(&numbers, third_last),
        now.elapsed().as_nanos()
    );
    let now = Instant::now();
    println!(
        "binary search for {third_last} in the list: {} - took {}ns",
        generic_search::binary_contains(&numbers, third_last),
        now.elapsed().as_nanos()
    );
}
