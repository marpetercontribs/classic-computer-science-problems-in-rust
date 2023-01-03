// fib2.rs
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

fn fib2(n: u32) -> u32 {
    if n<2 {
        // println!("fib({}) -> {}",n,n);
        n
    } else {
        // println!("fib({}) -> fib({}), fib({})",n,n-1,n-2);
        fib2(n-1) + fib2(n-2)        
    }
}

fn main() {
    println!("{}",fib2(5));
    println!("{}",fib2(10));
}
