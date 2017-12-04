#![feature(io)]

extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use itertools::Itertools;

fn key(word: &str) -> String {
    String::from_iter(word.chars().sorted())
}

fn is_valid(passphrase: &str) -> bool {
    let keys: Vec<String> = passphrase.split_whitespace().map(key).collect();
    keys.len() == keys.iter().unique().count()
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let all_passphrases = reader.lines().filter_map(|l| l.ok());
    let valid_passphrases = all_passphrases.filter(|p| is_valid(p));
    println!("Number of valid passphrases: {}", valid_passphrases.count());
}
