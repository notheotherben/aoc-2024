#[macro_use] pub mod helpers;
mod questions;

#[cfg(test)]
pub(crate) mod results;

fn main() {
    println!("Run 'cargo watch -x test' to run all of the solutions as you make changes.");
}
