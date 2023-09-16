# Classic Computer Science Problems in Rust

This repository contains my attempt at porting the source code of the books *Classic Computer Science Problems in Python* and *Classic Computer Science Problems in Java* by David Kopec to Rust.

You will find the source organized by chapter, treating each chapter as a package and each program as a binary crate within that package.
This means that each chapter was initialized using `cargo new chapter<number>`, so that it contains a single `src` folder (except for reused library-like code).  
The source files for the various executable programs have been created manually in subfolders `bin` using expressive names (similar to the Python edition of the book). Drawback is that they have to be built and executed using `cargo build --bin <name>` / `cargo run --bin <name>`.  
Reusable code, such as `generic_search` of chapter 2 and `csp` of chapter 3, are created as separate packages within the folder of the corresponding chapter. These packages contain a library crate only, with the source code in file `src/lib.rs`.

The alternative approach would be to treat each executable program/problem as a separate package within a chapter folder. Benefit would be that each program could be built and run easily using the usual `cargo build [--release]` / `cargo run` command within the program's folder. But it would also lead to a large number of additional *src* folders, *Cargo.toml* files, etc. In addition, the real source code files would have the name *main.rs*, not expressing the real names/intent of the programs.

Missing ports:

- Everything in Chapter 6.4ff

