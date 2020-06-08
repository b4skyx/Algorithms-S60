fn longest_palindromic_seq(arr: Vec<u8>) -> i32 {
    let alen = arr.len();
    let mut mem = vec![vec![0; alen]; alen];
    for i in 0..alen {
        mem[i][i] = 1;
    }
    for win in 2..alen + 1 {
        for i in 0..(alen - win + 1) {
            let j = i + win - 1;
            if arr[i] == arr[j] {
                if win == 2 {
                    mem[i][j] = 2;
                } else {
                    mem[i][j] = mem[i + 1][j - 1] + 2;
                }
            } else {
                mem[i][j] = std::cmp::max(mem[i][j - 1], mem[i + 1][j]);
            }
        }
    }
    mem[0][alen - 1] as i32
}
fn main() {
    println!("Hello, world!");
}
