use std::io;
fn min(arr: Vec<i32>, n: usize) -> i32 {
    if n >= arr.len() - 1 {
        println!("Too Large");
        return 0;
    }

    // Create a differene vec and fill the difference
    let mut diff_vec = vec![0; arr.len() - 1];
    for i in 0..(diff_vec.len() as usize) {
        diff_vec[i] = arr[i + 1] - arr[i];
    }

    let k = diff_vec.len() - n; // window_size of diff_vec
    let mut win_i: Vec<usize> = Vec::new(); //window vec to store index

    //push longest strictly dec order in win with largest at index 0 for frame 1
    for i in 0..k + 1 {
        while !win_i.is_empty() && diff_vec[i] >= diff_vec[win_i[0]] {
            win_i.pop();
        }
        win_i.push(i);
    }
    let mut res = diff_vec[win_i[0]];

    // move the window and check longest dec order
    for i in k..diff_vec.len() {
        //remove everything outside window
        while !win_i.is_empty() && win_i[0] < i - k {
            win_i.remove(0);
        }

        //remove elements if some lager difference is found
        while !win_i.is_empty() && diff_vec[i] >= diff_vec[*win_i.last().unwrap()] {
            win_i.pop();
        }
        win_i.push(i);

        //update result
        if diff_vec[win_i[0]] < res {
            res = diff_vec[win_i[0]];
        }
    }
    //check for last window
    if diff_vec[win_i[0]] < res {
        res = diff_vec[0];
    }
    return res;
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
