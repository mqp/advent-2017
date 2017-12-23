extern crate primal;

fn main() {
    println!("Number of composites: {}", (0..1001).filter(|n| !primal::is_prime(105700 + (17 * n) as u64)).count()); 
}