use std::io;
fn coin_change(elements: Vec<i64>, sum: usize) -> i64 {
    let alen = elements.len();
    let mut mem = vec![vec![0; sum + 1]; alen + 1];
    for i in 0..alen + 1 {
        for j in 0..sum + 1 {
            if j == 0 {
                mem[i][j] = 1;
            } else if i != 0 {
                if elements[i - 1] as usize <= j {
                    mem[i][j] = mem[i - 1][j] + mem[i][j - elements[i - 1] as usize];
                } else {
                    mem[i][j] = mem[i - 1][j];
                }
            }
        }
    }
    mem[alen][sum]
}
fn read_input() -> Vec<i64> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Failed to readline");
    ipt.split_whitespace()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
fn main() {
    let sum = read_input()[0];
    let elements = read_input();
    println!("{}", coin_change(elements, sum as usize));
}
