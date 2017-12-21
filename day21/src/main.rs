#![feature(io)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid<T> {
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.size() + x]
    }
    
    pub fn size(&self) -> usize {
        (self.data.len() as f64).sqrt() as usize // the grid is always square
    }

    pub fn map<F, U>(&self, func: F) -> Grid<U> where F: Fn(&T) -> U {
        Grid::new(self.data.iter().map(func).collect())
    }
}

impl<T> Grid<T> where T: Copy {
    pub fn rotate(&self) -> Self {
        let sz = self.size();
        let mut data = Vec::with_capacity(sz * sz);
        for y in 0..sz {
            for x in 0..sz {
                data.push(*self.get(sz - y - 1, x));
            }
        }
        Grid::new(data)
    }

    pub fn reflect(&self) -> Self {
        let sz = self.size();
        let mut data = Vec::with_capacity(sz * sz);
        for y in 0..sz {
            for x in 0..sz {
                data.push(*self.get(sz - 1 - x, y));
            }
        }
        Grid::new(data)
    }

    pub fn subgrid(&self, (xmin, ymin): (usize, usize), (xmax, ymax): (usize, usize)) -> Self {
        let mut data = Vec::with_capacity((ymax - ymin) * (xmax - xmin));
        for y in ymin..ymax {
            for x in xmin..xmax {
                data.push(*self.get(x, y));
            }
        }
        Grid::new(data)
    }

    pub fn merge(grids: Grid<Self>) -> Self {
        let sz = grids.size();
        let subgrid_size = grids.get(0, 0).size();
        let mut data = Vec::with_capacity(sz * sz * subgrid_size * subgrid_size);
        for gy in 0..sz {
            for y in 0..subgrid_size {
                for gx in 0..sz {
                    let subgrid = grids.get(gx, gy);
                    for x in 0..subgrid_size {
                        data.push(*subgrid.get(x, y));
                    }
                }
            }
        }
        Grid::new(data)
    }

    pub fn subdivide(&self) -> Grid<Self> {
        let sz = self.size();
        let d = **&[2, 3].iter().find(|&n| sz % n == 0).expect("Grid size must be a multiple of 2 or 3!");
        let n = sz / d;
        let mut data = Vec::with_capacity(n * n);
        for yi in 0..n {
            for xi in 0..n {
                data.push(self.subgrid((xi*d, yi*d), (xi*d + d, yi*d + d)));
            }
        }
        Grid::new(data)
    }
}

impl FromStr for Grid<bool> {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('/');
        let mut data = Vec::new();
        for p in parts {
            for ch in p.chars() {
                data.push(ch == '#');
            }
        }
        Ok(Grid::new(data))
    }
}

fn count(grid: &Grid<bool>) -> u32 {
    let sz = grid.size();
    let mut count = 0;
    for y in 0..sz {
        for x in 0..sz {
            if *grid.get(x, y) {
                count += 1;
            }
        }
    }
    count
}

fn apply_rule(rules: &HashMap<Grid<bool>, Grid<bool>>, input: &Grid<bool>) -> Grid<bool> {
    let r0 = input.clone();
    let r90 = r0.rotate();
    let r180 = r90.rotate();
    let r270 = r180.rotate();
    let r0x = r0.reflect();
    let r90x = r90.reflect();
    let r180x = r180.reflect();
    let r270x = r270.reflect();
    for candidate in &[r0, r0x, r90, r90x, r180, r180x, r270, r270x] {
        if let Some(output) = rules.get(candidate) {
            return output.clone();
        }
    }
    panic!("No rule found matching grid!");
}

fn parse_rule(s: &str) -> Result<(Grid<bool>, Grid<bool>), Box<Error>> {
    let mut parts = s.split(" => ");
    let precedent = parts.next().ok_or("No precedent in rule!")?.parse()?;
    let antecedent = parts.next().ok_or("No antecedent in rule!")?.parse()?;
    Ok((precedent, antecedent))
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines = reader.lines().map(|x| x.expect("Couldn't read line."));
    let rules: HashMap<Grid<bool>, Grid<bool>> = lines.map(|x| parse_rule(&x).expect("Couldn't parse rule.")).collect();
    let mut curr: Grid<bool> = ".#./..#/###".parse().unwrap();
    for _ in 0..18 {
        let curr_subgrids = curr.subdivide();
        let next_subgrids = curr_subgrids.map(|sg| apply_rule(&rules, &sg));
        curr = Grid::merge(next_subgrids);
    }
    println!("{}", count(&curr));
}
