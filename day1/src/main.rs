#![feature(io)]

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = File::open("input").expect("Couldn't read input file.");
    let digits: Vec<u32> = file.chars().filter_map(|ch| { ch.expect("Failed to read character.").to_digit(10) }).collect();
    let count = digits.len();
    let offset = count / 2;
    let mut total = 0;
    for i in 0..count {
        let a = digits[i];
        let b = digits[(i+offset) % count];
        if a == b {
            total += a;
        }
    }
    println!("Result is: {}", total);
}
