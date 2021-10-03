use lambda::Function;

mod lambda;
mod algorithms;

fn main() {
    let identity = lambda::Identity::new(12);
    println!("{:?}", identity.apply())
}