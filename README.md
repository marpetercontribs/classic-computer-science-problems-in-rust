# Classic Computer Science Problems in Rust

This repository contains my attempt at porting the source code of the books *Classic Computer Science Problems in Python* and *Classic Computer Science Problems in Java* by David Kopec to Rust.

You will find the source organized by chapter, treating each chapter as a package and each program as a binary crate within that package.
This means that each chapter was initialized using `cargo new chapter<number>`, so that it contains a single `src` folder (except for reused library-like code).  
The source files for the various executable programs have been created manually in subfolders `bin` using expressive names (similar to the Python edition of the book). Drawback is that they have to be built and executed using `cargo build --bin <name>` / `cargo run --bin <name>`.  
Reusable code, such as `generic_search` of chapter 2 and `csp` of chapter 3, are created as separate packages within the folder of the corresponding chapter. These packages contain a library crate only, with the source code in file `src/lib.rs`.

The alternative approach would be to treat each executable program/problem as a separate package within a chapter folder. Benefit would be that each program could be built and run easily using the usual `cargo build [--release]` / `cargo run` command within the program's folder. But it would also lead to a large number of additional *src* folders, *Cargo.toml* files, etc. In addition, the real source code files would have the name *main.rs*, not expressing the real names/intent of the programs.

Exercises completed:

- Chapter 1
    - [X] 1. Own Fibonacci algorithm with unit tests and performance comparison
    - [ ] 2. Specific to the programming language
    - [X] 3. Generlize hanoi to arbitrary number of towers
    - [ ] 4. One-time-pad to encrypt images
- Chapter 2
    - [X] 1. Demonstrate performance advantage of binary over linear search
    - [X] 2. Count the number of states covered in `bfs`, `dfs` and `astar` searches
    - [X] 3. Solve missionaries problem if the number of missionaries differs from the number of cannibals
- Chapter 3
    - [X] 1. Extend `word_search` to allow for words to cross
    - [ ] 2. Write a solver for problem 3.6
    - [X] 3. Write Sudoku solver
- Chapter 4
    - [X] 1. Add functions to remove edges and vertices 
    - [X] 2. Add support for di(rected) graphs
    - [ ] 3. Solve the Seven Bridges of Koenigsberg problem
- Chapter 5
    - [ ] 1. Add support for improved Tournament selection
    - [ ] 2. Extend CSP of chapter 3 by support for genetic algorithm
    - [ ] 3. Create a BitString that implements Chromosome and use it to solve 5.3
- Chapter 6
    - [ ] 1. Add support for importing DataPoints from a csv file
    - [ ] 2. Add function that generates a visualization of the (2D) points and clusters
    - [ ] 3. Add constructor for KMeans that allows specifying initial centroids
    - [ ] 4. Research and implement the kmeans++ algorithm
- Chapter 7
    - [ ] 1. Use the chapter's framework to classify another dataset
    - [ ] 2. Try a different activation function
    - [ ] 3. Solve this chapter's problems using a widely framework such as TensorFlow
    - [ ] 4. Rewrite Network, Layer and Neuron using a 3rd-party library optimizing numerical computations
- Chapter 8
    - [X] 1. Add unit tests to tictactoe to verify that methods get_legal_moves, is_win and is_draw are correct
    - [ ] 2. Add minimax unit tests for connectfour
    - [ ] 3. Refactor tictactoe_ai and connectfour_ai so that you have two methods usable for both games
    - [ ] 4. Change connectfour_ai to let the computer play against itself
    - [ ] 5. Can you optimize the evaluation method in connectfour (using profiling or other means) to increase the search depth without increasing runtime?
- Chapter 9
    - [ ] 1. Implement the naive approach to the traveling salesman problem using the graph framework of chapter 4 
    - [ ] 2. Implement a genetic algorithm as covered in chapter 5 to solve the traveling salesman problem
    - [ ] 3. Use a dictionary in the phone number mnemonics program to only return permutations containing "valid" words   
