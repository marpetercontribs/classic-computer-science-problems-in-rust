// calculating_pi.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 1
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

// use std::time::Instant; // remove comment to measure runtime

fn calculate_pi(n_terms: usize) -> f64 {
    let numerator = 4.0;
    let mut denominator = 1.0;
    let mut sign = 1.0;
    let mut pi = 0.0;
    for _ in 0..n_terms {
        pi += sign * (numerator / denominator);
        sign = -sign;
        denominator += 2.0
    }
    pi
}

fn main() {
    // let start = Instant::now();                        // remove comment to measure runtime
    // let pi = calculate_pi(1000000);                    // remove comment to measure runtime
    // let duration = start.elapsed().as_millis();        // remove comment to measure runtime
    // println!("Calculating {} took {}ms.",pi,duration); // remove comment to measure runtime
    println!("{}", calculate_pi(10000000))
}
