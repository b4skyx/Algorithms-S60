use std::io;
fn count_ways(n: i32) -> i32 {
    if n <= 1 {
        println!("{}", n);
        return n;
    }
    return 1 + count_ways(n - 1) + count_ways(n - 2);
}
fn main() {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Err");
    let ipt: i32 = ipt.trim().parse().unwrap_or(0);
    println!("{}", count_ways(ipt));
}
