use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn redistribute_one(banks: &mut Vec<u32>) {
    let count = banks.len();
    let (source_idx, source_blocks) = banks.iter()
        .enumerate()
        .max_by_key(|&(i, n)| (n, -(i as i32))) // largest value, smallest index breaks ties
        .map(|(i, n)| (i, *n))
        .unwrap();
        
    banks[source_idx] = 0;
    for i in 0..source_blocks {
        banks[(source_idx + 1 + i as usize) % count] += 1;
    }
}

fn redistribute_all(banks: &mut Vec<u32>) -> u32 {
    let mut seen: HashMap<Vec<u32>, u32> = HashMap::new();
    for i in 0.. {
        match seen.entry(banks.clone()) {
            Entry::Occupied(e) => { return i - e.get(); }
            Entry::Vacant(e) => { 
                e.insert(i);
                redistribute_one(banks);
            }
        }
    }
    unreachable!("The loop must terminate.");
}

fn main() {
    let input = include_str!("../input");
    let mut banks: Vec<u32> = input.split_whitespace().filter_map(|l| l.parse().ok()).collect();
    let steps = redistribute_all(&mut banks);
    println!("Number of steps: {}", steps);
}
