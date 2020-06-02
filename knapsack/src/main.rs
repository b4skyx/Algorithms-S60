use std::io;
fn knapsack(elements: Vec<i32>, sum: usize) -> bool {
    let alen = elements.len();
    let mut mem = vec![vec![false; sum + 1]; alen + 1];
    for i in 0..alen + 1 {
        for j in 0..sum + 1 {
            if j == 0 {
                mem[i][j] = true;
            } else if i == 0 && j != 0 {
                mem[i][j] = false;
            } else if elements[i - 1] as usize <= j {
                mem[i][j] = std::cmp::max(mem[i - 1][j], mem[i - 1][j - elements[i - 1] as usize]);
            } else {
                mem[i][j] = mem[i - 1][j];
            }
        }
    }
    for i in 0..alen + 1 {
        println!("{:?}", mem[i]);
    }
    mem[alen][sum]
}
fn read_input() -> Vec<i32> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Failed to readline");
    ipt.split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
fn main() {
    let sum = read_input()[0];
    let elements = read_input();
    println!("{}\t", knapsack(elements, sum as usize));
}
