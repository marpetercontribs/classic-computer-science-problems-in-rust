// fib5.rs
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

pub fn fib5(n: usize) -> usize {
    if n==0 {
        n
    } else {
        let (mut last, mut next) = (0, 1);
        for _ in 1..n {
            (last, next) = (next, last + next);
        }
        next
    }
}

fn main() {
    println!("{}", fib5(5));
    println!("{}", fib5(50));
}
