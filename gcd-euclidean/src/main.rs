// Finding GCD by euclidean algorithm.
// Recursive Approach

use std::io;
fn gcd(a: &u64, b: &u64) -> u64 {
    if b == &0 {
        return *a;
    }
    gcd(&b, &(a % b))
}
fn main() {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Failed");
    let ipt = ipt
        .split_whitespace()
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    println!(
        "{}",
        gcd(
            std::cmp::max(&ipt[0], &ipt[1]),
            std::cmp::min(&ipt[0], &ipt[1])
        )
    )
}
