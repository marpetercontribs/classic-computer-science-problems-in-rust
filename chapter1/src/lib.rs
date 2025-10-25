// lib.rs
// Adapted for Rust version
// of Classic Computer Science Problems in Python/Java Chapter 1
//
// This code does not directly correspond to any of the problems covered in
// the books, but to have a file structure similar to Kopec's (on github)
// compatible with rust's package manager cargo, with one folder per chapter
// without having to change the generated Cargo.toml for each program
//   of chapter 1 in the books, and
// without having a seperate rust package and sub-folder for each program
//   with its own src folder and generic file name main.rs
//
// Copyright 2025 Markus Peter
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

pub mod fib2 {
    pub fn fib2(n: usize) -> usize {
        if n < 2 {
            n
        } else {
            fib2(n - 1) + fib2(n - 2)
        }
    }
}

pub mod fib3 {
    pub struct Fib3 {
        memo: std::collections::HashMap<usize, usize>,
    }

    impl Fib3 {
        pub fn new() -> Self {
            let mut memo = std::collections::HashMap::<usize, usize>::new();
            memo.insert(0, 0); // Part 1 of the recursion's stop condition
            memo.insert(1, 1); // Part 2 of the recursion's stop condition
            Fib3 { memo }
        }
        pub fn get(&mut self, n: usize) -> usize {
            if !self.memo.contains_key(&n) {
                let prev_prev = self.get(n - 2);
                let prev = self.get(n - 1);
                self.memo.insert(n, prev + prev_prev);
            }
            *self.memo.get(&n).unwrap()
        }
    }

    impl Default for Fib3 {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub mod fib4 {
    use memoize::memoize;

    // The following line requires dependency memoize in Cargo.toml
    #[memoize]
    pub fn fib4(n: usize) -> usize {
        if n < 2 {
            n
        } else {
            fib4(n - 1) + fib4(n - 2)
        }
    }
}

pub mod fib5 {
    pub fn fib5(n: usize) -> usize {
        if n == 0 {
            n
        } else {
            let (mut last, mut next) = (0, 1);
            (1..n).for_each(|_| (last, next) = (next, last + next));
            next
        }
    }
}

pub mod fib6 {
    pub struct FibonacciGenerator {
        last: usize,
        next: usize,
        max: usize,
        done: usize,
    }

    impl FibonacciGenerator {
        fn new(n: usize) -> Self {
            FibonacciGenerator {
                last: 0,
                next: 1,
                max: n,
                done: 0,
            }
        }
    }

    impl Iterator for FibonacciGenerator {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            if self.done < self.max {
                self.done += 1;
                let result = Some(self.last);
                (self.last, self.next) = (self.next, self.last + self.next);
                result
            } else {
                None
            }
        }
    }

    pub fn fib6(n: usize) -> FibonacciGenerator {
        FibonacciGenerator::new(n)
    }
}
