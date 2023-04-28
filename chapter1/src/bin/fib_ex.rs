// fib_ex.rs
// Solution to exercise 1 
// in Classic Computer Science Problems in Python/Java Chapter 1
// This solution simply uses the additional towers to "park" one of the top
// elements of the first tower at the beginning
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

mod fib2;
mod fib3;
mod fib4;
mod fib5;
mod fib6;

struct FibEx {
    memo: Vec<usize>
}

impl FibEx {
    fn new() -> Self {
        FibEx { memo: vec![0,1] }
    }
    fn get(&mut self, n: usize) -> usize {
        if n < self.memo.len() {
            return self.memo[n];
        } 
        if n == self.memo.len() {
            let result = self.memo[n-1] + self.memo[n-2];
            self.memo.push(result);
            return result;
        } else {
            return self.get(n-1) + self.get(n-2);
        }

    }
}

fn compare(n: usize) {
    let now = Instant::now();
    println!("Simple recursive algorithm returns:    fib({n})={} - took {}ns", fib2::fib2(n), now.elapsed().as_nanos());
    let now = Instant::now();
    println!("Caching recursive algorithm returns:   fib({n})={} - took {}ns", fib3::Fib3::new().get(n), now.elapsed().as_nanos());
    let now = Instant::now();
    println!("Memoizing recursive algorithm returns: fib({n})={} - took {}ns", fib4::fib4(n), now.elapsed().as_nanos());
    let now = Instant::now();
    println!("Loop with pair algorithm returns:      fib({n})={} - took {}ns", fib5::fib5(n), now.elapsed().as_nanos());
    let now = Instant::now();
    let mut fib = 0;
    for i in fib6::fib6(n+1) {
        fib = i;
    }
    println!("Generator algorithm returns:           fib({n})={} - took {}ns", fib, now.elapsed().as_nanos());
    let now = Instant::now();
    println!("Own algorithm returns:                 fib({n})={} - took {}ns", FibEx::new().get(n), now.elapsed().as_nanos());
}
fn main() {
    compare(5);
    println!("---------------------------------------------------");
    compare(10);
    println!("---------------------------------------------------");
    compare(20);
    println!("---------------------------------------------------");
    compare(40);
    println!("---------------------------------------------------");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_cases() {
        assert_eq!(FibEx::new().get(0),0);
        assert_eq!(FibEx::new().get(1),1);
    }

    #[test]
    fn fib_5_is_5() {
        assert_eq!(FibEx::new().get(5),5);
    }
    #[test]
    fn fib_10_is_55() {
        assert_eq!(FibEx::new().get(10),55);
    }
}