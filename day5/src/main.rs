#![feature(io)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn step(instructions: &mut Vec<i32>, at: usize) -> Option<usize> {
    let instr = instructions[at];
    let next = at as i32 + instr;
    let delta = if instr >= 3 { -1 } else { 1 };
    instructions[at] = instr + delta;
    if next >= 0 && (next as usize) < instructions.len() {
        Some(next as usize)
    } else {
        None
    }
}

fn execute(instructions: &mut Vec<i32>) -> u32 {
    let mut steps = 0;
    let mut curr = 0;
    loop {
        steps += 1;
        if let Some(next) = step(instructions, curr) {
            curr = next; 
        } else {
            return steps;
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines = reader.lines().filter_map(|l| l.ok());
    let mut instructions: Vec<i32> = lines.filter_map(|l| l.parse().ok()).collect();
    let steps = execute(&mut instructions);
    println!("Number of steps: {}", steps);
}
