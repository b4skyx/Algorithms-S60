use std::io;
fn min(arr: Vec<i32>, n: usize) -> i32 {
    if n >= arr.len() - 1 {
        println!("Too Large");
        return 0;
    }
    let mut diff_vec = vec![0; arr.len() - 1];
    // Create a differene vec and fill the difference
    for i in 0..(diff_vec.len() as usize) {
        diff_vec[i] = arr[i + 1] - arr[i];
    }

    let k = diff_vec.len() - n; // window_size of diff_vec
    let mut k_win: Vec<usize> = Vec::new(); //window vec

    //push longest strictly dec order in win with largest at index 0
    for i in 0..k {
        while !k_win.is_empty() && diff_vec[i] >= diff_vec[k_win[0]] {
            k_win.pop();
        }
        k_win.push(i);
    }

    let mut res = i32::max_value();

    // move the window and check longest dec order
    for i in k..diff_vec.len() {
        //update result
        if diff_vec[0] < res {
            res = diff_vec[0];
        }

        //remove everything outside window
        while !k_win.is_empty() && k_win[0] <= i - k {
            k_win.remove(0);
        }

        //remove elements if some lager difference is found
        while !k_win.is_empty() && diff_vec[i] >= diff_vec[*k_win.last().unwrap()] {
            k_win.pop();
        }
        k_win.push(i);
    }
    //check for last window
    if diff_vec[0] < res {
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
