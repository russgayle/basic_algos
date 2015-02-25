extern crate basic_algos;
extern crate rand;

use rand::{thread_rng, sample};

#[allow(dead_code)]
fn main() {
    println!("Hello, world!");

    let v = 5;
    basic_algos::silly::print_int(v);
    basic_algos::silly::print_int(34i32);

    let mut rng = thread_rng();
    let sampled_set = sample(&mut rng, 1..100, 10);
    println!("{:?}", sampled_set);
}
