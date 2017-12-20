#![feature(io)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use regex::Regex;
use std::collections::HashMap;
use std::ops::{Add, AddAssign};

lazy_static! {
    static ref PARTICLE_RE: Regex = Regex::new(r#"p=<(?P<p>.+)>, v=<(?P<v>.+)>, a=<(?P<a>.+)>"#).unwrap();
    static ref VECTOR3_RE: Regex = Regex::new(r#"(?P<x>\-?\d+),(?P<y>\-?\d+),(?P<z>\-?\d+)"#).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = *self + other;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Particle {
    pub position: Vector3,
    pub velocity: Vector3,
    pub acceleration: Vector3,
}

fn timestep(particles: &mut Vec<Particle>) {
    for particle in particles {
        particle.velocity += particle.acceleration;
        particle.position += particle.velocity;
    }
}

fn collide(particles: &mut Vec<Particle>) {
    let mut positions = HashMap::new();
    for particle in particles.iter() {
        positions.entry(particle.position).or_insert_with(Vec::new).push(*particle);
    }
    particles.retain(|p| positions.get(&p.position).unwrap().len() == 1);
}

fn parse_vector(s: &str) -> Result<Vector3, Box<Error>> {
    let parts = VECTOR3_RE.captures(s).ok_or("Failed to parse vector.")?;
    Ok(Vector3 {
        x: parts.name("x").unwrap().as_str().parse()?,
        y: parts.name("y").unwrap().as_str().parse()?,
        z: parts.name("z").unwrap().as_str().parse()?,
    })
}

fn parse_particle(s: &str) -> Result<Particle, Box<Error>> {
    let properties = PARTICLE_RE.captures(s).ok_or("Failed to parse particle.")?;
    Ok(Particle {
        position: parse_vector(properties.name("p").unwrap().as_str())?,
        velocity: parse_vector(properties.name("v").unwrap().as_str())?,
        acceleration: parse_vector(properties.name("a").unwrap().as_str())?,
    })
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines = reader.lines().map(|x| x.expect("Couldn't read line."));
    let mut particles: Vec<Particle> = lines.map(|x| parse_particle(&x).expect("Couldn't parse line.")).collect();
    let original_count = particles.len();
    loop {
        timestep(&mut particles);
        collide(&mut particles);
        println!("{} {}", original_count, particles.len());
    }
}
