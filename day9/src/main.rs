#![feature(io)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn consume_garbage<T>(input: &mut T) -> u32 where T: Iterator<Item=char> {
    let mut size = 0;
    while let Some(ch) = input.next() {
        match ch {
            '!' => { input.next(); }
            '>' => { return size; },
            _ => { size += 1; }
        };
    }
    panic!("Non-terminated garbage in input!");
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let mut input = reader.chars().map(|ch| ch.expect("Couldn't parse file."));
    let mut score = 0;
    while let Some(ch) = input.next() {
        match ch {
            '<' => { score += consume_garbage(&mut input); },
            '{' => {
                // depth += 1;
            },
            '}' => {
                // score += depth;
                // depth -= 1;
            },
            _ => {},
        };
    }
    println!("Total score is: {}", score);
}
