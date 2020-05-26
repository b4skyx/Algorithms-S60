use std::io;
fn min(arr: Vec<i32>, n: u32) {
    unimplemented!();
}

fn read_input() -> Vec<i32> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Failed to ReadLine");
    ipt.split_ascii_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
fn main() {
    let input = read_input();
    let k = read_input()[0];
    min(input, k as u32);
}
