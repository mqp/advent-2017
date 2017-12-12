use std::fmt::Write;

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

fn main() {
    let mut chars: Vec<u8> = include_str!("../input").trim().into();
    let mut suffix = vec![17u8, 31u8, 73u8, 47u8, 23u8];
    chars.append(&mut suffix);

    let output = hashify(&chars, 64);
    let mut hash = String::new();
    for byte in output {
        write!(&mut hash, "{:02x}", byte).expect("Unable to write to output.");
    }
    println!("Hash is: {}", hash);
}
