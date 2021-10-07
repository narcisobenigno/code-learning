extern crate rand;

use functional::Function;

mod functional;
mod algorithms;

fn main() {
    let identity = functional::Identity::new(12);
    println!("{:?}", identity.apply())
}