// A program that emulates the behavior of trees in animal crossing: new horizons
// in Rust!

use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{io, io::Write};

// define the random number generator
fn create_rng() -> ThreadRng {
    let rng: ThreadRng = thread_rng();
    return rng;
}

struct Tree {
    // should i make the rng a member of this struct?
    num_branches: u8,
}

impl Tree {
    fn new() -> Self {
        let mut rng: ThreadRng = create_rng();
        let new_num_branches: u8 = rng.gen_range(1..=5);
        return Self {
            num_branches: new_num_branches,
        };
    }

    fn shake(&mut self) -> u8 {
        let mut rng: ThreadRng = create_rng(); // should i make this rng a member of Tree?
        let dice_roll: f64 = rng.gen::<f64>();
        if dice_roll > 0.65 {
            self.num_branches -= 1;
            return 1;
        } else {
            return 0;
        }
    }

    fn get_num_branches(&self) -> u8 {
        return self.num_branches;
    }
}

fn main() {
    // lets create a new tree and see how many branches it has!
    let mut fresh_tree: Tree = Tree::new();
    println!("This tree has {} branches.", fresh_tree.get_num_branches());
    print!("Do you want to shake the tree to get its branches? \ntype y\\n: ");
    // emit to stdout immediately
    io::stdout().flush().expect("failed to flush stdout.");

    let mut start_shaking: String = String::new();
    io::stdin()
        .read_line(&mut start_shaking)
        .expect("failed to read input.");

    if start_shaking.eq("y\n") {
        while fresh_tree.get_num_branches() > 0 {
            let branches_dropped: u8 = fresh_tree.shake();
            if branches_dropped > 0 {
                println!("You got a branch!\n");
            } else {
                println!("You didn't get any branches...\n");
            }

            print!("Press the Enter key to shake the tree again!: ");
            // emit to stdout immediately
            io::stdout().flush().expect("failed to flush stdout.");

            let mut response: String = String::new();
            io::stdin()
            .read_line(&mut response)
            .expect("failed to read input.");
        }

        println!("\nThis tree doesn't have any more branches.")
    } else {
        println!("\nYou didn't want to shake the tree...")
    }
}
