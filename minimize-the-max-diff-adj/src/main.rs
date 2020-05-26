use std::io;
fn min(arr: Vec<i32>, n: u32) {
    let mut k = n.clone();
    let mut front = 0usize;
    let mut last = arr.len() - 1;
    if (last - front) < k as usize {
        println!("K is too Large");
    } else {
        while k > 0 {
            if (arr[front + 1] - arr[front]) > (arr[last] - arr[last - 1]) {
                front += 1;
            } else {
                last -= 1;
            }
            k -= 1
        }
        let mut max_diff = 0;
        for i in front..last {
            if max_diff < arr[i + 1] - arr[i] {
                max_diff = arr[i + 1] - arr[i];
            }
        }
        println!("{}", max_diff);
    }
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
    min(input, k as u32);
}
