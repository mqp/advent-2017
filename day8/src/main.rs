#![feature(io)]
#![feature(try_from)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use std::error::Error;
use std::collections::HashMap;
use std::str::FromStr;
use std::convert::TryFrom;

lazy_static! {
    static ref INSTRUCTION_RE: Regex = Regex::new(
        r#"(?P<reg>\w+) (?P<op>\w+) (?P<val>\-?\d+) if (?P<target>\w+) (?P<cmp>.+) (?P<threshold>\-?\d+)"#
    ).unwrap();
}

#[derive(Debug, Copy, Clone)]
enum Op { Inc, Dec }

impl FromStr for Op {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Op::Inc),
            "dec" => Ok(Op::Dec),
            _ => Err(From::from("Invalid operation.")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Cmp { LessThan, LessEqual, Equal, NotEqual, GreaterThan, GreaterEqual }

impl FromStr for Cmp {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Cmp::LessThan),
            "<=" => Ok(Cmp::LessEqual),
            "==" => Ok(Cmp::Equal),
            "!=" => Ok(Cmp::NotEqual),
            ">=" => Ok(Cmp::GreaterEqual),
            ">" => Ok(Cmp::GreaterThan),
            _ => Err(From::from("Invalid comparison operator.")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction<'a> {
    pub reg: &'a str,
    pub op: Op,
    pub val: i32,
    pub target: &'a str,
    pub cmp: Cmp,
    pub threshold: i32,
}

impl<'a> TryFrom<&'a str> for Instruction<'a> {    
    type Error = Box<Error>;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let parts = INSTRUCTION_RE.captures(s).ok_or("Failed to parse line.")?;
        let reg = parts.name("reg").unwrap().as_str();
        let op = parts.name("op").unwrap().as_str().parse()?;
        let val = parts.name("val").unwrap().as_str().parse()?;
        let target = parts.name("target").unwrap().as_str();
        let cmp = parts.name("cmp").unwrap().as_str().parse()?;
        let threshold = parts.name("threshold").unwrap().as_str().parse()?;
        Ok(Instruction { reg, op, val, target, cmp, threshold })
    }
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let entries = lines.iter().map(|l| Instruction::try_from(l.as_str()).expect("Couldn't parse line."));
    let mut registers = HashMap::new();
    let mut max_val = 0;
    for entry in entries {
        let register_val = *registers.get(entry.reg).unwrap_or(&0);
        let target_val = *registers.get(entry.target).unwrap_or(&0);
        let meets_condition = match entry.cmp {
            Cmp::LessThan => target_val < entry.threshold,
            Cmp::LessEqual => target_val <= entry.threshold,
            Cmp::Equal => target_val == entry.threshold,
            Cmp::NotEqual => target_val != entry.threshold,
            Cmp::GreaterEqual => target_val >= entry.threshold,
            Cmp::GreaterThan => target_val > entry.threshold,
        };
        if meets_condition {
            let delta = match entry.op {
                Op::Inc => entry.val,
                Op::Dec => -entry.val,
            };
            let new_val = register_val + delta;
            max_val = i32::max(new_val, max_val);
            registers.insert(entry.reg, new_val);
        }
    }
    println!("Maximum value was: {:?}", max_val);
}
