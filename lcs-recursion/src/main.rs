use std::io;
fn max(a: i32, b: i32) -> i32 {
    if a >= b {
        return a;
    } else {
        return b;
    }
}
fn lcs(a: &Vec<u8>, b: &Vec<u8>, table: &mut Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    if m == 0 || n == 0 {
        table[m][n] = 0;
        return table[m][n];
    } else if table[m][n] != -1 {
        return table[m - 1][n - 1];
    } else if a[m - 1] == b[n - 1] {
        table[m - 1][n - 1] = 1 + lcs(&a, &b, table, m - 1, n - 1);
        return table[m - 1][n - 1];
    } else {
        table[m - 1][n - 1] = i32::max(lcs(&a, &b, table, m, n - 1), lcs(&a, &b, table, m - 1, n));
        return table[m - 1][n - 1];
    }
}
fn read_input() -> Vec<u8> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("error");
    ipt.trim()
        .as_bytes()
        .iter()
        .map(|x| *x)
        .collect::<Vec<u8>>()
}
fn main() {
    let a = read_input();
    let b = read_input();
    let m = a.len();
    let n = b.len();
    let mut mem = vec![vec![-1; b.len() + 1]; a.len() + 1];
    println!("{}", lcs(&a, &b, &mut mem, m, n));
}
