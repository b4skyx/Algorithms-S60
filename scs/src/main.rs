struct Solution {}
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let a = str1.as_bytes();
        let b = str2.as_bytes();
        let mut alen = a.len();
        let mut blen = b.len();
        let mut res = Vec::new();

        let mut mem = vec![vec![-1; blen + 1]; alen + 1];
        for i in 0..(alen as usize + 1) {
            for j in 0..(blen as usize + 1) {
                if i == 0 || j == 0 {
                    mem[i][j] = 0;
                } else if a[i - 1] == b[j - 1] {
                    mem[i][j] = mem[i - 1][j - 1] + 1;
                } else {
                    mem[i][j] = i32::max(mem[i][j - 1], mem[i - 1][j]);
                }
            }
        }
        for i in 0..alen + 1 {
            println!("{:?}", mem[i]);
        }
        while alen > 0 || blen > 0 {
            if alen > 0 && blen > 0 && a[alen - 1] == b[blen - 1] {
                res.push(a[alen - 1]);
                alen -= 1;
                blen -= 1;
            } else if blen > 0 && mem[alen][blen - 1] == mem[alen][blen] {
                res.push(b[blen - 1]);
                blen -= 1;
            } else if alen > 0 && mem[alen - 1][blen] == mem[alen][blen] {
                res.push(a[alen - 1]);
                alen -= 1;
            } else {
                if alen > 0 {
                    res.push(a[alen - 1]);
                    alen -= 1;
                }
                if blen > 0 {
                    res.push(b[blen - 1]);
                    blen -= 1;
                }
            }
        }
        res.reverse();
        String::from_utf8(res).unwrap()
    }
}
fn main() {
    lcs(
        String::from("abcd").chars().collect::<Vec<char>>(),
        String::from("cab").chars().collect::<Vec<char>>(),
    );
    println!("Hello, world!");
}
