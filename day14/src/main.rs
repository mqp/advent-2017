extern crate itertools;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn pinch(xs: &mut [u8], pos: usize, length: usize) {
    let count = length / 2;
    for offset in 0..count {
        let i = (pos + offset) % xs.len();
        let j = (pos + length - offset - 1) % xs.len();
        xs.swap(i, j);
    }
}

fn xor(xs: &[u8]) -> u8 {
    let mut output = 0;
    for x in xs {
        output ^= x;
    }
    output
}

fn compact(xs: &[u8], block_size: usize) -> Vec<u8> {
    xs.chunks(block_size).map(xor).collect()
}

fn initialize(length: usize) -> Vec<u8> {
    let mut xs = Vec::with_capacity(length);
    for i in 0..length {
        xs.push(i as u8);
    }
    xs
}

fn hashify(input: &[u8], rounds: u32) -> Vec<u8> {
    let mut skip = 0;
    let mut pos = 0;
    let mut sparse = initialize(256);
    for _ in 0..rounds {
        for x in input {
            let length = *x as usize;
            pinch(&mut sparse, pos, length);
            pos += skip + length;
            skip += 1;
        }
    }
    compact(&mut sparse, 16)
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub nodes: HashSet<(i32, i32)>,
}

fn color_all(grid: &Grid) -> HashMap<(i32, i32), u32> {
    let mut colors = HashMap::new();
    let mut next_color = 0;
    for node in grid.nodes.iter() {
        if !colors.contains_key(node) {
            colors.insert(*node, next_color);
            color_connected(&grid, &mut colors, &node, next_color);
            next_color += 1;
        }
    }
    colors
} 

fn color_connected(grid: &Grid, colors: &mut HashMap<(i32, i32), u32>, to: &(i32, i32), color: u32) {
    let &(x0, y0) = to;
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for &(xd, yd) in dirs.iter() {
        let neighbor = (x0 + xd, y0 + yd);
        if grid.nodes.contains(&neighbor) && !colors.contains_key(&neighbor) {
            colors.insert(neighbor, color);
            color_connected(grid, colors, &neighbor, color);
        }
    }
    
}

fn bit_set(input: u8, n: usize) -> bool {
    input & (1 << n) != 0
}

fn main() {
    let prefix = "hxtvlmkl";
    let mut grid = Grid { nodes: HashSet::new() };
    for n in 0..128 {
        let mut input: Vec<u8> = format!("{}-{}", prefix, n).into();
        let mut suffix = vec![17u8, 31u8, 73u8, 47u8, 23u8];
        input.append(&mut suffix);

        let hash = hashify(&input, 64);   
        for (i, byte) in hash.iter().enumerate() {
            for j in 0..8 {
                if bit_set(*byte, 7 - j) {
                    grid.nodes.insert(((i as i32) * 8 + (j as i32), n));
                }
            }
        }
    }
    let colors = color_all(&grid);
    let regions = colors.values().unique().count();
    println!("Total is: {}", regions);
}
