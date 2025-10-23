// connectfour_ai.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 8
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
use chapter8::board::Board;
use chapter8::connectfour::C4Board;
use chapter8::minimax::find_best_move;
use std::io;

fn get_player_move(board: &C4Board) -> usize {
    let mut player_move: usize = 100;
    while !board.legal_moves().contains(&player_move) {
        let mut input = String::new();
        println!("Enter a legal column (0-6):");
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading your input.");
        player_move = input.trim().parse().expect("Enter a legal column (0-6)!");
        if board.legal_moves().contains(&player_move) {}
    }
    player_move
}

fn main() {
    let mut board = C4Board::new();
    loop {
        let human_move: usize = get_player_move(&board);
        board = board.do_move(human_move);
        println!("{board}");
        if board.is_win() {
            println!("Human wins!");
            break;
        } else if board.is_draw() {
            println!("Draw!");
            break;
        }
        let computer_move: usize = find_best_move(&board, 6).expect("Cannot find best move!");
        println!("The computer move is {}.", computer_move);
        board = board.do_move(computer_move);
        println!("{board}");
        if board.is_win() {
            println!("Computer wins!");
            break;
        } else if board.is_draw() {
            println!("Draw!");
            break;
        }
    }
}
