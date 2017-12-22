#![feature(io)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeState {
    Weakened,
    Infected,
    Flagged,
}

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
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
        }
    }
    fn turn_cw(&self) -> Self {
        match *self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Up => Direction::Right,
        }
    }
    fn turn_ccw(&self) -> Self {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
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

fn parse_grid(s: &str) -> HashMap<(i32, i32), NodeState> {
    let mut data = HashMap::new();
    let rows: Vec<&str> = s.trim().split_whitespace().collect();
    let height = rows.len();
    let y_offset = (height / 2) as i32;
    for (y, row) in rows.iter().enumerate() {
        let width = row.trim().len();
        let x_offset = (width / 2) as i32;
        for (x, ch) in row.trim().chars().enumerate() {
            match ch {
                '#' => { data.insert((x as i32 - x_offset, y as i32 - y_offset), NodeState::Infected); }
                '.' => {}
                _ => panic!("Unrecognized character in input.")
            }            
        }
    }
    data
}

fn main() {
    let mut grid = parse_grid(&include_str!("../input"));
    let mut n_infected = 0;
    let mut dir = Direction::Up;
    let mut coords = (0, 0);
    for _ in 0..10000000 {
        match grid.entry(coords) {
            Entry::Vacant(entry) => {
                dir = dir.turn_ccw();
                entry.insert(NodeState::Weakened);
            }
            Entry::Occupied(mut entry) => {
                match entry.get() {
                    &NodeState::Weakened => {
                        n_infected += 1;
                        *entry.get_mut() = NodeState::Infected;
                    }
                    &NodeState::Infected => {
                        dir = dir.turn_cw();
                        *entry.get_mut() = NodeState::Flagged;
                    }
                    &NodeState::Flagged => {
                        dir = dir.reverse();
                        entry.remove();
                    }
                }
            }
        }
        let delta = dir.delta();
        coords = (coords.0 + delta.0, coords.1 + delta.1);
    }
    println!("{}", n_infected);
}
