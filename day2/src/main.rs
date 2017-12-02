#![feature(io)]

extern crate itertools;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn distance(line: String) -> u32 {
    let entries: Vec<u32> = line.split_whitespace().filter_map(|n| n.parse().ok()).collect();
    let result = entries.iter().tuple_combinations::<(_, _)>().find(|&(x, y)| { x % y == 0 || y % x == 0 });
    let (x, y) = result.expect("No evenly divisible entries.");
    return if x > y { x / y } else { y / x };
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let distances = reader.lines().map(|l| distance(l.expect("Couldn't read input line.")));
    println!("Result is: {}", distances.sum::<u32>());
}
