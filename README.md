# Classic Computer Science Problems in Python

This repository contains my attempt at portin the source code of the books *Classic Computer Science Problems in Python* and *Classic Computer Science Problems in Java* by David Kopec to Rust.

You will find the source organized by chapter.

For now, each individually executable program is created as a separate package.
This means that each individually executable program was created using `cargo new <name>` within the folder of the corresponding chapter, resulting in a further subfolder named *\<name\>* containing its own *Cargo.toml* file, a subfolder *src* and the real program code in file **main.rs** therein.

This has the benefit that each program can be created easily and independently from the others. It can also be built and run easily using the usual `cargo buld [--release]` / `cargo run` commands within the program's folder (as the subfolder of the corresponding chapter).

Drawbacks are the large number of additional *src* folders, *Cargo.toml* files and that the real source code files have the name *main.rs*, not expressing the real name of the program.

Consider switching to a single package per chapter, with different "binaries" listed in the *Cargo.toml* file and stored subfolder in *src/bin*. They would have to be built using `cargo build --bin <name>` / `cargo run --bin <name>`.

Missing ports:

- Chapter 1.1: Generator version (equivalent of fib6 in Python / fib5 in Java book)
