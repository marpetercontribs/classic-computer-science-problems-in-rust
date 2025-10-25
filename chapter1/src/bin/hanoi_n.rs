// hanoi_n.rs
// Solution to exercise 3
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

use std::fmt;
use std::slice::IterMut;

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
        self.stack
            .pop()
            .expect("You cannot pop from an empty stack!")
    }
}

impl fmt::Display for Stack<u8> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[").expect("Cannot write to formatter!?");
        if !self.stack.is_empty() {
            for index in 0..self.stack.len() - 1 {
                write!(formatter, "{}, ", self.stack[index]).expect("Cannot write to formatter!?");
            }
            write!(formatter, "{}", self.stack[self.stack.len() - 1])
                .expect("Cannot write to formatter!?");
        }
        write!(formatter, "]")
    }
}

fn move_elements<T>(begin: &mut Stack<T>, end: &mut Stack<T>, temp: &mut Stack<T>, n: usize) {
    if n == 1 {
        end.push(begin.pop());
    } else {
        move_elements(begin, temp, end, n - 1);
        move_elements(begin, end, temp, 1);
        move_elements(temp, end, begin, n - 1);
    }
}

fn solve_hanoi(
    begin: &mut Stack<u8>,
    end: &mut Stack<u8>,
    temp: &mut Stack<u8>,
    remaining: IterMut<'_, Stack<u8>>,
) {
    let len = begin.stack.len();
    let mut park_count = 0;
    let mut parking_towers: Vec<&mut Stack<u8>> = Vec::<_>::new();
    for parking_tower in remaining {
        parking_tower.push(begin.pop());
        parking_towers.push(parking_tower);
        park_count += 1;
    }
    move_elements(begin, end, temp, len - park_count);
    for parking_tower in parking_towers.iter_mut().rev() {
        end.push(parking_tower.pop());
    }
}

fn main() {
    let tower_height = 6;
    let tower_count = 6;
    let mut towers: Vec<Stack<u8>> = Vec::<_>::new();
    for _ in 0..tower_count {
        towers.push(Stack::new());
    }
    for i in 1..=tower_height {
        towers[0].push(i)
    }

    for (i, item) in towers.iter().enumerate() {
        println!("Tower {}: {}", i + 1, item);
    }

    let mut towers_remaining = towers.iter_mut();
    let tower_a = towers_remaining.next().unwrap();
    let tower_b = towers_remaining.next().unwrap();
    let tower_c = towers_remaining.next().unwrap();

    solve_hanoi(tower_a, tower_c, tower_b, towers_remaining);

    for (i, item) in towers.iter().enumerate() {
        println!("Tower {}: {}", i + 1, item);
    }
}
