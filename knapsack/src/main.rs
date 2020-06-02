fn knapsack(elements: Vec<u32>, sum: usize) -> bool {
    let alen = elements.len();
    let mut mem = vec![vec![false; sum + 1]; alen + 1];
    for i in 0..alen + 1 {
        for j in 0..sum + 1 {
            if j == 0 {
                mem[i][j] = true;
            } else if i == 0 && j != 0 {
                mem[i][j] = false;
            } else if elements[i - 1] as usize <= j {
                mem[i][j] = std::cmp::max(mem[i - 1][j], mem[i - 1][j - elements[i - 1] as usize]);
            } else {
                mem[i][j] = mem[i - 1][j];
            }
        }
    }
    mem[alen][sum]
}
fn main() {
    println!("Hello, world!");
}
