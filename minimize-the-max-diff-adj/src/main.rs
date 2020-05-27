use std::io;

fn min(arr: Vec<i32>, n: usize) -> i32 {
    if n >= arr.len() - 1 {
        println!("Too large");
        return 0;
    }
    let mut diff_vec = Vec::new();
    for i in 0..(arr.len() - 1) {
        diff_vec.push(arr[i + 1] - arr[i]);
    }
    let k = diff_vec.len() - n; // window_size
    let mut ki_win: Vec<usize> = Vec::new();
    for i in 0..k {
        while !ki_win.is_empty() && diff_vec[ki_win[0]] < diff_vec[i] {
            ki_win.pop();
        }
        ki_win.push(i);
    }
    let mut res = diff_vec[ki_win[0]];
    for i in k..diff_vec.len() {
        while !ki_win.is_empty() && ki_win[0] < i - k {
            ki_win.remove(0);
        }
        while !ki_win.is_empty() && diff_vec[*ki_win.last().unwrap()] <= diff_vec[i] {
            ki_win.pop();
        }
        ki_win.push(i);
        if res > diff_vec[ki_win[0]] {
            res = diff_vec[ki_win[0]];
        }
    }
    res
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
    println!("{}", min(input, k as usize));
}
