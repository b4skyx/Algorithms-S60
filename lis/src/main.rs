use std::io;
struct Solution {}
impl Solution {
    pub fn length_of_lis(seq: Vec<i32>) -> i32 {
        let mut mem = vec![1; seq.len()];
        for i in 0..seq.len() {
            for j in 0..i {
                if seq[i] > seq[j] {
                    mem[i] = i32::max(mem[i], mem[j] + 1);
                }
            }
        }
        *mem.iter().max().unwrap_or(&0)
    }
}
fn read_input() -> Vec<i32> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Err");
    ipt.split_ascii_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
fn main() {
    println!("{}", Solution::length_of_lis(read_input()));
}
