use std::collections::HashMap;
use std::io;
fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed");
    let n = n.trim().parse::<u32>().unwrap();
    for _ in 0..n {
        let mut a = String::new();
        let mut b = String::new();
        io::stdin().read_line(&mut a).expect("Input Err");
        io::stdin().read_line(&mut b).expect("Input Err");
        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        let mut count = 0;
        for ch in a.chars() {
            let counter = m1.entry(ch).or_insert(0);
            *counter += 1;
        }
        for ch in b.chars() {
            let counter = m2.entry(ch).or_insert(0);
            *counter += 1;
        }
        if m1 == m2 {
            println!("0");
            continue;
        }
        for ch in m1.keys() {
            if m2.contains_key(ch) {
                count += (m1[ch] - m2[ch] as i32).abs();
            } else {
                count += m1[ch];
            }
        }
        for ch in m2.keys().filter(|x| !m1.contains_key(x)) {
            count += m2[ch];
        }
        println!("{}", count);
    }
}
