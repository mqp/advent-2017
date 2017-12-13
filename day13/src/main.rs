#![feature(io)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::collections::HashMap;

fn scanner_position(range: u32, t: u32) -> u32 {
    let span = (range - 1) * 2;
    let offset = t % span;
    if offset > range - 1 {
        span - offset
    } else {
        offset
    }
}

fn caught(firewall: &HashMap<u32, u32>, delay: u32) -> bool {
    let max_layer = firewall.keys().max().unwrap();
    for depth in 0..(max_layer + 1) {
        if let Some(range) = firewall.get(&depth) {
            if scanner_position(*range, depth + delay) == 0 {
                return true;
            }
        }
    }
    false
}

fn parse(line: &str) -> Result<(u32, u32), Box<Error>> {
    let parts: Vec<&str> = line.split(": ").collect();
    if parts.len() != 2 {
        Err(From::from("Failed to parse line."))
    } else {
        Ok((parts[0].parse()?, parts[1].parse()?))
    }
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let firewall: HashMap<u32, u32> = lines.iter().map(|l| parse(l).expect("Couldn't parse line.")).collect();
    let first_uncaught_delay = (0..).find(|n| !caught(&firewall, *n)).unwrap();
    println!("Best delay: {}", first_uncaught_delay);
}
