use std::io;
fn lcs(a: Vec<char>, b: Vec<char>) {
    let alen = a.len();
    let blen = b.len();

    let mut mem = vec![vec![-1; blen + 1]; alen + 1];
    for i in 0..(alen as usize + 1) {
        for j in 0..(blen as usize + 1) {
            if i == 0 || j == 0 {
                mem[i][j] = 0;
            } else if a[i - 1] == a[j - 1] {
                mem[i][j] = mem[i - 1][j - 1] + 1;
            } else {
                mem[i][j] = i32::max(mem[i][j - 1], mem[i - 1][j]);
            }
        }
    }
    println!("{}", mem[alen][blen]);
}
fn read_input() -> Vec<char> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("error");
    ipt.trim().chars().collect::<Vec<char>>()
}
fn main() {
    let a = read_input();
    let b = read_input();
    lcs(a, b);
}
