#![feature(io)]

use std::str::FromStr;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::error::Error;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Port {
    pub px: u32,
    pub py: u32,
}

impl Port {
    fn score(&self) -> u32 {
        self.px + self.py
    }
    fn fits(&self, pin: u32) -> Option<u32> {
        if self.px == pin {
            Some(self.py)
        } else if self.py == pin {
            Some(self.px)
        } else {
            None
        }
    }
}

impl FromStr for Port {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('/');
        let px = parts.next().ok_or("No pin count specified!")?.parse()?;
        let py = parts.next().ok_or("No pin count specified!")?.parse()?;
        Ok(Port { px, py })
    }
}

fn greedy_max(ports: &Vec<Port>, from_pin: u32) -> (u32, u32) {
    let mut best = (0, 0);
    for (i, port) in ports.iter().enumerate() {
        if let Some(next_pin) = port.fits(from_pin) {
            let mut ports_copy = ports.clone();
            ports_copy.swap_remove(i);
            let (len, score) = greedy_max(&ports_copy, next_pin);
            best = cmp::max(best, (1 + len, port.score() + score));
        }
    }
    best
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines = reader.lines().map(|x| x.expect("Couldn't read line."));
    let ports: Vec<Port> = lines.map(|x| x.parse().expect("Couldn't parse port.")).collect();
    println!("Best: {:?}", greedy_max(&ports, 0));
}
