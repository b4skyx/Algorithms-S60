use std::io;
fn read_input() -> Vec<u64> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Err");
    ipt.trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}
fn main() {
    let t = read_input()[0];
    for _ in 0..t {
        let n = read_input()[0];
        let mut a = read_input();
        let mut b = read_input();
        a.sort();
        b.sort();
        let mut res = 0;
        let mut j = 0;
        for i in 0..n as usize {
            if b[i] > a[j] {
                j += 1;
                res += 1;
            }
        }
        println!("{}", res);
    }
}
