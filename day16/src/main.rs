#![feature(io)]
#![feature(slice_rotate)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

impl FromStr for Move {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, data) = s.split_at(1);
        match id {
            "s" => Ok(Move::Spin(data.parse().expect("Couldn't parse move argument."))),
            "x" => {
                let mut parts = data.split('/');
                Ok(Move::Exchange(parts.next().unwrap().parse()?, parts.next().unwrap().parse()?))
            }
            "p" => {
                let mut parts = data.split('/');
                Ok(Move::Partner(parts.next().unwrap().chars().next().unwrap(), parts.next().unwrap().chars().next().unwrap()))
            }
            _ => Err(From::from("Unknown move provided."))
        }

    }
}

fn next(programs: &mut [u8], m: Move) {
    match m {
        Move::Spin(x) => { let len = programs.len(); programs.rotate(len - x); }
        Move::Exchange(i, j) => programs.swap(i, j),
        Move::Partner(a, b) => {
            let ai = programs.iter().position(|x| *x == a as u8).expect("Invalid program name specified.");
            let bi = programs.iter().position(|x| *x == b as u8).expect("Invalid program name specified.");
            programs.swap(ai, bi);
        },
    };
}

fn find_period(moves: &Vec<Move>) -> u64 {
    let mut programs: Vec<u8> = (0..16).map(|n| ('a' as u8) + n).collect();
    let initial_programs = programs.clone();

    for i in 1.. {
        for x in moves {
            next(&mut programs, *x);
        }
        if programs == initial_programs {
            return i;
        }
    }
    unreachable!("The loop must terminate.");
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let parts = reader.split(b',').map(|x| String::from_utf8(x.expect("Couldn't parse file.")).unwrap());
    let moves: Vec<Move> = parts.map(|x| x.trim().parse().expect("Couldn't parse move.")).collect();

    let mut programs: Vec<u8> = (0..16).map(|n| ('a' as u8) + n).collect();
    let period = find_period(&moves);
    let remainder = 1000000000 % period;
    for _ in 0..remainder {
        for x in &moves {
            next(&mut programs, *x);
        }
    }

    println!("Period: {:?}, Programs: {:?}", period, String::from_utf8(programs));
}
