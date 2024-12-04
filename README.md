# Advent of Code 2024
**My solutions to the Advent of Code 2023 puzzles.**

This year I'm using [Rust](https://www.rust-lang.org/) to solve the Advent of Code puzzles, with
a particular emphasis on getting comfortable writing Rust code quickly and efficiently. In my
typical fashion, I'm emphasizing readability and maintainability over code-golfing, and you'll
also find that I'm leaning heavily on `cargo test` to run my solutions (specifically,
the entire project is intended to be run under `cargo watch -x test` for continuous feedback
during development).

## Goals
 - **Continuous Testing** - All solutions run continuously under `cargo watch -x test` and full test
    runs complete in < 1s, making it easy to get immediate feedback on the correctness of any
    changes.

 - **Standard Rust** - I've tried to stick exclusively with the standard library rather than pulling in
    external crates. This is mostly to keep the project simple and easy to build, but also to
    encourage me to learn the standard library better.

 - **Fast** - I've tried to ensure that my solutions are as fast as possible, aiming to keep the total
    runtime significantly under 1 second and being careful to ensure that I'm
    implementing intelligent solutions to the problems.