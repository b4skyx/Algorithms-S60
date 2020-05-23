// Two Sum problem
// https://www.hackerearth.com/practice/data-structures/hash-tables/basics-of-hash-tables/practice-problems/algorithm/pair-sums/

use std::collections::HashSet;
use std::io;

fn two_sum(n: Vec<i32>, sum: i32) -> String {
    let mut set: HashSet<i32> = HashSet::new();
    for ele in n {
        if set.contains(&(sum - ele)) {
            println!("{} {}", ele, sum - ele);
            return String::from("YES");
        } else {
            set.insert(ele);
        }
    }
    return String::from("NO");
}

fn read_input() -> Vec<i32> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Failed to readline");
    ipt.split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
fn main() {
    let s = read_input()[1];
    let n = read_input();
    println!("{}", two_sum(n, s));
}
