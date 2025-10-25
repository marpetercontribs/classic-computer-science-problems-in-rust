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

use chapter1::fib3::Fib3;

fn main() {
    let mut fib: Fib3 = Default::default(); // Fib3::new();
                                            // Note the need for `mut` here, because the get method changes the state of Fib3.
                                            // To avoid this, the HashMap could be wrapped inside a std::rc::RefCell.
    println!("{}", fib.get(5));
    println!("{}", fib.get(50));
}
