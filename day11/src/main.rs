use std::str::FromStr;
use std::error::Error;
use std::cmp;

#[derive(Debug, Clone, Copy)]
enum HexDirection {
    N, NW, SW, S, SE, NE
}

impl FromStr for HexDirection {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(HexDirection::N),
            "nw" => Ok(HexDirection::NW),
            "sw" => Ok(HexDirection::SW),
            "s" => Ok(HexDirection::S),
            "se" => Ok(HexDirection::SE),
            "ne" => Ok(HexDirection::NE),
            _ => Err(From::from("Invalid direction string.")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct HexPoint {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

fn add(point: HexPoint, dir: HexDirection) -> HexPoint {
    let (xd, yd, zd) = match dir {
        HexDirection::N  => (1, 0, -1),
        HexDirection::NW => (1, -1, 0),
        HexDirection::SW => (0, -1, 1),
        HexDirection::S  => (-1, 0, 1),
        HexDirection::SE => (-1, 1, 0),
        HexDirection::NE => (0, 1, -1),
    };
    HexPoint { x: point.x + xd, y: point.y + yd, z: point.z + zd }
}

fn distance(a: HexPoint, b: HexPoint) -> u32 {
    (i32::abs(a.x - b.x) + i32::abs(a.y - b.y) + i32::abs(a.z - b.z)) as u32 / 2
}

fn main() {
    let dirs = include_str!("../input").split(',').filter_map(|x| x.parse().ok());
    let origin = HexPoint { x: 0, y: 0, z: 0 };
    let mut position = origin;
    let mut max_distance = 0;
    for dir in dirs {
        position = add(position, dir);
        max_distance = cmp::max(max_distance, distance(origin, position));
    }
    println!("MAx distance from origin: {}", max_distance);
}
