use std::io;
fn prime_c(range: u32) {
    let mut isprime: Vec<bool> = vec![true; range as usize + 1];
    let mut prime_vec = vec![2];
    for num in (3..(range as usize + 1)).step_by(2) {
        if isprime[num] == true {
            for i in ((num * 2)..range as usize).step_by(num) {
                isprime[i] = false;
            }
            prime_vec.push(num);
        }
    }
    println!("{:?}", prime_vec);
    println!("{}", prime_vec.len());
}

fn prime_v(range: u32) {
    let mut isprime: Vec<bool> = vec![true; range as usize + 1];
    // let mut prime_vec: Vec<usize> = Vec::new();
    let mut count = 0u32;
    for num in 2..(range as usize + 1) {
        if isprime[num] == true {
            for i in ((num * num)..range as usize + 1).step_by(num) {
                isprime[i] = false;
            }
            count += 1;
            println!("{}", num);
            // prime_vec.push(num);
        }
    }
    println!("{}", count);
    // println!("{:?}", prime_vec);
    // println!("{}", prime_vec.len());
}

fn main() {
    let mut range = String::new();
    io::stdin()
        .read_line(&mut range)
        .expect("Error reading input");
    let range = range.trim().parse::<u32>().unwrap();
    // prime_c(range.clone());
    prime_c(range);
}
