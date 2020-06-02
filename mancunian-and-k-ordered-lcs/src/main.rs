use std::io;

fn lcs(a: Vec<u32>, b: Vec<u32>) -> u32 {
    let alen = a.len();
    let blen = b.len();
    let mut mem = vec![vec![-1; blen + 1]; alen + 1];
    for i in 0..alen + 1 {
        for j in 0..blen + 1 {
            if i == 0 || j == 0 {
                mem[i][j] = 0;
            } else if a[i - 1] == b[j - 1] {
                mem[i][j] = mem[i - 1][j - 1] + 1;
            } else {
                mem[i][j] = std::cmp::max(mem[i - 1][j], mem[i][j - 1]);
            }
        }
    }
    mem[alen][blen] as u32
}

fn read_input() -> Vec<u32> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("error");
    ipt.trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}
fn main() {
    let ipt = read_input();
    let a = read_input();
    let mut b = read_input();
    for _ in 0..(ipt[1] as usize - &b.len()) {
        b.push(read_input()[0]);
    }
    println!("{}", lcs(a, b) + ipt[2] as u32);
}
