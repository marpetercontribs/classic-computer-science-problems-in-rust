// hanoi.rs
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

use std::fmt;

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, element: T) {
        self.stack.push(element);
    }

    fn pop(&mut self) -> T {
        self.stack.pop().expect("You cannot pop from an empty stack!")
    }
}

impl fmt::Display for Stack<u8> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[").expect("Cannot write to formatter!?");
        if self.stack.len() > 0 {
            for index in 0..self.stack.len()-1 {
                write!(formatter, "{}, ", self.stack[index]).expect("Cannot write to formatter!?");
            }
            write!(formatter, "{}", self.stack[self.stack.len()-1]).expect("Cannot write to formatter!?"); 
        }
        write!(formatter,"]")
    }
}

fn move_elements<T>(begin: &mut Stack<T>, end: &mut Stack<T>, temp: &mut Stack<T>, n: usize) {
    if n==1 {
        end.push( begin.pop() );
    } else {
        move_elements(begin, temp, end, n-1);
        move_elements(begin, end, temp, 1);
        move_elements(temp, end, begin, n-1);
    }
}

fn solve_hanoi(begin: &mut Stack<u8>, end: &mut Stack<u8>, temp: &mut Stack<u8>) {
    let len = begin.stack.len();
    move_elements(begin, end, temp, len);
}

fn main() {
    let tower_height = 3;
    let mut tower_a: Stack<u8> = Stack::new();
    let mut tower_b: Stack<u8> = Stack::new();
    let mut tower_c: Stack<u8> = Stack::new();
    for i in 1..=tower_height { tower_a.push(i) };

    solve_hanoi(&mut tower_a, &mut tower_c, &mut tower_b);

    println!("Tower A: {}",tower_a);
    println!("Tower B: {}",tower_b);
    println!("Tower C: {}",tower_c);
}
