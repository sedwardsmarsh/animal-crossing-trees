// A program that emulates the behavior of trees in animal crossing: new horizons
// in Rust!

use std::thread::Thread;

use rand::{thread_rng, Rng, rngs::ThreadRng};


// define the random number generator
fn createRng() -> ThreadRng {
    let mut rng: ThreadRng = thread_rng();
    return rng
}


struct Tree {
    num_branches: u8
}


impl Tree {
    fn new() -> Self {
        let mut rng: ThreadRng = createRng();
        let new_num_branches: u8 = rng.gen_range(1..=5);
        return Self{num_branches: new_num_branches};
    }

    // fn shake(&mut self) -> u8 {
    //     // create our random number generator
    //     let mut rng: ThreadRng = createRng();
    //     let dice_roll: f64 = rng.gen::<f64>();
    //     if dice_roll > 0.65 {
    //         self
    //     }
    // }

    fn getNumBranches(&self) -> u8 {
        return self.num_branches;
    }
}


fn main() {
    // lets create a new tree and see how many branches it has!
    let mut fresh_tree: Tree = Tree::new();
    println!("This tree has {} branches.", fresh_tree.getNumBranches());
}