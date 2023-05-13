// fib4.rs
// Adapted From Classic Computer Science Problems in Python Chapter 1
// Classic Computer Science Problems in Java does not include an equivalent version
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

use memoize::memoize;

#[memoize]

pub fn fib4(n: usize) -> usize {
    if n<2 {
        n
    } else {
        fib4(n - 1) + fib4(n - 2)
    }
}

fn main() {
    println!("{}", fib4(5));
    println!("{}", fib4(50));
}
