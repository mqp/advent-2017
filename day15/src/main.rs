const MASK: u64 = 0b1111_1111_1111_1111;

fn step(factor: u64, x: u64) -> u64 {
    (x * factor) % 2147483647
}

fn main() {
    let af = 16807;
    let bf = 48271;
    let mut a = 679;
    let mut b = 771;
    let mut matches = 0;
    let rounds = 5000000;
    for _ in 0..rounds {
        a = step(af, a);
        b = step(bf, b);
        while a % 4 != 0 {
            a = step(af, a);
        }
        while b % 8 != 0 {
            b = step(bf, b);
        }
        if a & MASK == b & MASK {
            matches += 1;
        }
    }
    println!("Total is: {}", matches);
}
