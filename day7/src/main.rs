#![feature(io)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use std::error::Error;
use std::collections::HashMap;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(
        r#"(?P<prog>\w+) \((?P<weight>\d+)\)( -> (?P<links>.*))?"#
    ).unwrap();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Id<'a>(pub &'a str);

#[derive(Debug, Copy, Clone)]
struct Node<'a> {
    pub id: Id<'a>,
    pub weight: u32,
}

#[derive(Debug, Clone)]
struct Tree<'a> {
    pub nodes: HashMap<Id<'a>, Node<'a>>,
    pub edges: HashMap<Id<'a>, Vec<Id<'a>>>,
}

fn total_weight<'a>(tree: &'a Tree, root: Id<'a>) -> u32 {
    let node = tree.nodes.get(&root).unwrap();
    match tree.edges.get(&root) {
        None => node.weight,
        Some(xs) => node.weight + xs.iter().map(|x| { total_weight(tree, *x) }).sum::<u32>()
    }
}

fn weigh_subtrees<'a>(tree: &'a Tree, subtrees: &'a Vec<Id>) -> HashMap<u32, Vec<Id<'a>>> {
    let mut weights: HashMap<u32, Vec<Id>> = HashMap::new(); // {subtree_weight: [subtree]}
    for subtree in subtrees {
        weights.entry(total_weight(tree, *subtree)).or_insert_with(Vec::new).push(*subtree);
    }
    weights
}

fn find_unbalanced<'a>(tree: &'a Tree, root: Id<'a>) -> (Id<'a>, u32) { 
    // find the single subtree with a different weight, then call find_unbalanced_in_subtree
    // on it with the weight we know it's supposed to have
    let subtrees = tree.edges.get(&root).expect("Root must have children!");
    let weights = weigh_subtrees(tree, subtrees);
    let (_, subtree) = weights.iter().find(|&(_k, v)| v.len() == 1).expect("Whole tree is balanced!");
    let (target_weight, _) = weights.iter().find(|&(_k, v)| v.len() > 1).expect("No unique solution!");
    find_unbalanced_in_subtree(tree, subtree[0], *target_weight)
}

fn find_unbalanced_in_subtree<'a>(tree: &'a Tree, root: Id<'a>, target_weight: u32) -> (Id<'a>, u32) {
    match tree.edges.get(&root) {
        None => (root, target_weight), // no subtrees; this node needs the right weight
        Some(subtrees) => {
            // does one of the subtrees have a different weight than the others?
            let weights = weigh_subtrees(tree, subtrees);
            let total_subtree_weights: u32 = weights.iter().map(|(k, v)| *k * v.len() as u32).sum();
            match weights.iter().find(|&(_k, v)| v.len() == 1) {
                None => (root, target_weight - total_subtree_weights), // all subtrees have same weight; must be this node's fault
                Some((_, bad_subtree)) => {
                    let (target_weight, _) = weights.iter().find(|&(_k, v)| v.len() > 1).expect("No unique solution!");
                    find_unbalanced_in_subtree(tree, bad_subtree[0], *target_weight)
                }
            }
        }
    }
}

fn parse(line: &str) -> Result<(Node, Vec<Id>), Box<Error>> {
    let parts = LINE_RE.captures(line).ok_or("Failed to parse line.")?;
    let id = Id(parts.name("prog").unwrap().as_str());
    let weight = parts.name("weight").unwrap().as_str().parse()?;
    let links = match parts.name("links") {
        Some(text) => text.as_str().split(", ").map(Id).collect(),
        None => Vec::new() 
    };
    Ok((Node { id, weight }, links))
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let entries = lines.iter().map(|l| parse(l).expect("Couldn't parse line."));
    let mut tree = Tree { nodes: HashMap::new(), edges: HashMap::new() };
    for (program, links) in entries {
        tree.edges.insert(program.id, links);
        tree.nodes.insert(program.id, program);
    }
    println!("Correct weight is: {:?}", find_unbalanced(&tree, Id("wiapj")));
}
