mod task16;
use crate::task16::{prob1, prob2};
use std::env;
use std::fs;
use std::time::Instant;

const DAY: u8 = 16;

fn main() {
    let args: Vec<_> = env::args().collect();
    let inp = match args[2].as_str() {
        "main" => {
            fs::read_to_string(format!("inputs/task{}/main.txt", DAY)).expect("File not found")
        }
        "example" => {
            fs::read_to_string(format!("inputs/task{}/example.txt", DAY)).expect("File not found")
        }
        _ => panic!("Unknown target"),
    };
    let now = Instant::now();
    match args[1].as_str() {
        "1" => println!("{}", prob1(&inp)),
        "2" => println!("{}", prob2(&inp)),
        _ => panic!("Unknown part"),
    }
    println!("Time spent: {:.2?}", now.elapsed());
}
