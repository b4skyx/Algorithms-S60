use std::collections::HashMap;
use std::io;
fn char_push(exchange: &mut HashMap<char, char>) {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Error");
    let mut ipt = ipt
        .to_lowercase()
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect::<Vec<char>>();
    let mut p0 = false;
    let mut p1 = false;
    if !exchange.contains_key(&ipt[0]) {
        p0 = true;
    }
    if !exchange.contains_key(&ipt[1]) {
        p1 = true;
    }
    if p0 == false || p1 == false {
        for (_, v) in exchange.iter_mut() {
            if v == &mut ipt[0] {
                *v = ipt[1];
            } else if v == &mut ipt[1] {
                *v = ipt[0];
            }
        }
    }
    if p0 {
        exchange.insert(ipt[0], ipt[1]);
    }
    if p1 {
        exchange.insert(ipt[1], ipt[0]);
    }
}
fn main() {
    let mut exchange: HashMap<char, char> = HashMap::new();
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("1");
    let n = n.trim().parse::<u32>().unwrap();

    for _ in 0..n {
        char_push(&mut exchange);
    }

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("2");
    for ch in line.chars() {
        if let Some(x) = exchange.get(&ch.to_ascii_lowercase()) {
            if ch.is_uppercase() {
                print!("{}", x.to_ascii_uppercase());
            } else {
                print!("{}", x);
            }
        } else {
            print!("{}", &ch);
        }
    }
}
