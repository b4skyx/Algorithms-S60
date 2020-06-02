use std::io;
fn cumsum(a: Vec<i32>, k: i32) -> i32 {
    let mut cum_arr = Vec::new();
    let mut temp = 0;
    for ele in a {
        temp += ele;
        cum_arr.push(temp);
    }
    let mut rem_arr = Vec::new();
    println!("{:?}", rem_arr);
    for ele in cum_arr {
        rem_arr.push((ele % k + k) % k);
    }
    rem_arr.push(0);
    let mut hashtable = vec![0; k as usize];
    for ele in rem_arr {
        hashtable[ele as usize] += 1;
    }
    let mut res = 0;
    for ele in hashtable {
        res += (ele * (ele - 1)) / 2;
    }
    res
}
fn read_input() -> Vec<i32> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Err");
    ipt.split_ascii_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
fn main() {
    let a = read_input();
    let k = read_input()[0];
    println!("{}", cumsum(a, k));
}
