// fib6.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 1
// Support for generators in Rust is experimental.
// This solution uses an Iterator implementation to mimick a Python-like Generator
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
struct FibonacciGenerator {
    last: usize,
    next: usize,
    max: usize,
    done: usize
}

impl FibonacciGenerator {
    fn new(n: usize) -> Self {
        FibonacciGenerator { last: 0, next: 1, max: n, done: 0 }
    }
}

impl Iterator for FibonacciGenerator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done < self.max {
            self.done += 1;
            let result = Some(self.last);
            (self.last, self.next) = (self.next, self.last + self.next);
            return result;
        } else {
            return None;
        }
    }
}

fn fib6(n: usize) -> FibonacciGenerator {
    FibonacciGenerator::new(n)
}

fn main() {
    for i in fib6(50) {
        println!("{}",i);
    }
}