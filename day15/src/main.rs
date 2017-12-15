#[derive(Debug, Clone)]
struct LCG {
    factor: u64,
    current: u64
}

impl LCG {
    fn new(factor: u64, seed: u64) -> Self { 
        Self { factor, current: seed }
    }
}

impl Iterator for LCG {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.current = (self.current * self.factor) % 2147483647;
        Some(self.current)
    }
}

const MASK: u64 = 0b1111_1111_1111_1111;

fn main() {
    let a = LCG::new(16807, 679);
    let b = LCG::new(48271, 771);
    let mut xs = a.filter(|x| x % 4 == 0);
    let mut ys = b.filter(|y| y % 8 == 0);
    let mut matches = 0;
    for _ in 0..5000000 {
        if xs.next().unwrap() & MASK == ys.next().unwrap() & MASK {
            matches += 1;
        }
    }
    println!("Total is: {}", matches);
}
