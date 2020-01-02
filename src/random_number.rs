extern crate rand;
use rand::Rng;

pub fn run(){

    let random_number = rand::thread_rng().gen_range(1, 11);
    println!("the random number is {}", random_number);

    let random_bool = rand::thread_rng().gen_weighted_bool(2);
    println!("random bool: {}", random_bool);
}