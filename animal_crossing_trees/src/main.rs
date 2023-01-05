// A program that emulates the behavior of trees in animal crossing: new horizons
// in Rust!

use rand::{thread_rng, Rng};

fn main() {
    // initialize the random number generator
    let mut rng = thread_rng();

    // the equals before the 10 defines an inclusive range (10 included)
    let random_number: u32 = rng.gen_range(5..=10);
    println!("Here's a random number: {random_number}");
}