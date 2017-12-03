use std::cmp;
use std::collections::HashMap;

fn neighbors(grid: &HashMap<(i32, i32), u32>, (x, y): (i32, i32)) -> Vec<u32> {
    let mut result = Vec::with_capacity(8);
    for dx in &[-1, 0, 1] {
        for dy in &[-1, 0, 1] {
            if *dx != 0 || *dy != 0 {
                if let Some(val) = grid.get(&(x + dx, y + dy)) {
                    result.push(*val);
                }
            }
        }
    }
    result
}

fn mark(grid: &mut HashMap<(i32, i32), u32>, (x, y): (i32, i32)) -> u32 {
    let val = cmp::max(neighbors(grid, (x, y)).iter().sum(), 1);
    grid.insert((x, y), val);
    val
}

fn populate_until(threshold: u32) -> HashMap<(i32, i32), u32> {
    let mut grid = HashMap::new();
    let (mut  x, mut  y) = (0,  0);
    let (mut dx, mut dy) = (0, -1);
    loop {
        if mark(&mut grid, (x, y)) > threshold {
            break;
        }
        // take care to handle the bottom right; see last clause
        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let tmp = dy;
            dy = dx;
            dx = -tmp;
        }
        x += dx;
        y += dy;
    }
    grid
}

fn main() {
    let grid = populate_until(368078);
    println!("Maximum value: {:?}", grid.values().max());
}