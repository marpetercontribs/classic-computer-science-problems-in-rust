
// fib3.rs
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

use std::collections::HashMap;

struct Fib3 {
    memo: HashMap::<u32,u32>
}

impl Fib3 {
    fn new() -> Self {
        let mut memo = HashMap::<u32,u32>::new() ;
        memo.insert(0,0); // Part 1 of the recursion's stop condition
        memo.insert(1,1); // Part 2 of the recursion's stop condition       
        Fib3 { memo: memo }
    }
    fn get(&mut self, n: u32) -> u32 {
        if !self.memo.contains_key(&n) {
            let prev_prev = self.get(n-2);
            let prev = self.get(n-1);
            self.memo.insert(n,prev + prev_prev);
        }
 //       self.memo.entry(n).or_insert(self.fib(n-1)+self.fib(n-2));
        *self.memo.get(&n).unwrap()
    }
}

fn main() {
    let mut fib = Fib3::new(); // How to adjust code to avoid `mut` here?
    println!("{}",fib.get(5));
    println!("{}",fib.get(40));
}
