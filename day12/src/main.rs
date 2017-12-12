#![feature(io)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r#"(?P<node>\w+) <-> (?P<others>.*)"#).unwrap();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Id<'a>(pub &'a str);

#[derive(Debug, Clone)]
struct Graph<'a> {
    pub edges: HashMap<Id<'a>, HashSet<Id<'a>>>,
}

fn color_all<'a>(graph: &Graph<'a>) -> HashMap<Id<'a>, u32> {
    let mut colors = HashMap::new();
    let mut next_color = 0;
    for node in graph.edges.keys() {
        if colors.get(node).is_none() {
            colors.insert(*node, next_color);
            color_connected(&mut colors, graph, node, next_color);
            next_color += 1;
        }
    }
    colors
} 

fn color_connected<'a>(colors: &mut HashMap<Id<'a>, u32>, graph: &Graph<'a>, to: &Id<'a>, color: u32) {
    if let Some(others) = graph.edges.get(to) {
        for other in others {
            if colors.get(other).is_none() {
                colors.insert(*other, color);
                color_connected(colors, graph, other, color);
            }
        }
    }
}

fn parse(line: &str) -> Result<(Id, Vec<Id>), Box<Error>> {
    let parts = LINE_RE.captures(line).ok_or("Failed to parse line.")?;
    let node = parts.name("node").unwrap().as_str();
    let others = parts.name("others").unwrap().as_str();
    Ok((Id(node), others.split(", ").map(|n| Id(n)).collect()))
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let entries = lines.iter().map(|l| parse(l).expect("Couldn't parse line."));
    let mut graph = Graph { edges: HashMap::new() };
    for (from, nodes) in entries {
        for to in nodes {
            graph.edges.entry(from).or_insert_with(HashSet::new).insert(to);
            graph.edges.entry(to).or_insert_with(HashSet::new).insert(from);
        }
    }
    let colors = color_all(&graph);
    println!("Number of groups: {}", colors.values().unique().count());
}
