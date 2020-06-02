use std::collections::HashMap;
use std::io;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn longest(len: usize, seq: Vec<u64>) {
    let mut mem = vec![1; len];
    let mut gcd_map: HashMap<(u64, u64), u64> = HashMap::new();
    for i in 0..seq.len() {
        for j in 0..i {
            let g;
            if gcd_map.contains_key(&(seq[j], seq[i])) {
                g = gcd_map[&(seq[j], seq[i])]
            } else {
                g = gcd(seq[j], seq[i]);
                gcd_map.insert((seq[j], seq[i]), g);
            }
            if g > 1 {
                mem[i] = u32::max(mem[i], mem[j] + 1);
            }
        }
    }
    println!("{}", mem.iter().max().unwrap());
}
fn read_input() -> Vec<u64> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Err");
    ipt.split_ascii_whitespace()
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}
fn main() {
    longest(read_input()[0] as usize, read_input());
}
