#![feature(io)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn reverse(&self) -> Self {
        match *self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
    fn delta(&self) -> (i32, i32) {
        match *self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down  => (0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    Corner,
    Marker(char),
}

impl Tile {
    fn from(ch: char) -> Option<Self> {
        match ch {
            ' ' => None,
            '|' => Some(Tile::Vertical),
            '-' => Some(Tile::Horizontal),
            '+' => Some(Tile::Corner),
            _ => Some(Tile::Marker(ch))
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    pub nodes: HashMap<(i32, i32), Tile>,
}

fn find_start(grid: &Grid) -> (i32, i32) {
    if let Some((start, _)) = grid.nodes.iter().find(|&(&(_, y), &t)| t == Tile::Vertical && y == 0) {
        *start
    } else {
        panic!("No starting tile specified!");
    }
}

fn walk(grid: &Grid) -> (u32, String) {
    let mut result = String::new();
    let mut curr = find_start(grid);
    let mut dir = Direction::Down;
    let mut steps = 0;
    loop {
        let (dx, dy) = dir.delta();
        match grid.nodes.get(&curr) {
            None => {
                return (steps, result);
            }
            Some(&Tile::Marker(ch)) => {
                steps += 1;
                curr = (curr.0 + dx, curr.1 + dy);
                result.push(ch);
            }
            Some(&Tile::Vertical) | Some(&Tile::Horizontal) => {
                steps += 1;
                curr = (curr.0 + dx, curr.1 + dy);
            }
            Some(&Tile::Corner) => {
                for candidate in &[Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
                    if *candidate != dir.reverse() {
                        let (cx, cy) = candidate.delta();
                        let next_tile = (curr.0 + cx, curr.1 + cy);
                        if grid.nodes.contains_key(&next_tile) {
                            dir = *candidate;
                            curr = next_tile;
                            steps += 1;
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines = reader.lines().map(|x| x.expect("Couldn't read line."));
    let mut grid = Grid { nodes: HashMap::new() };
    for (y, line) in lines.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if let Some(tile) = Tile::from(ch) {
                grid.nodes.insert((x as i32, y as i32), tile);
            }
        }
    }
    let result = walk(&grid);
    println!("Result is: {:?}", result);
}
