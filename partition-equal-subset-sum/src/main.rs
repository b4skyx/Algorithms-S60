struct Solution {}
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let alen = nums.len();
        let mut sum = nums.iter().fold(0, |acc, x| acc + x);
        if sum % 2 != 0 {
            false
        } else {
            let sum = (sum / 2) as usize;
            let mut mem = vec![vec![false; sum + 1]; alen + 1];
            for i in 0..alen + 1 {
                for j in 0..sum + 1 {
                    if j == 0 {
                        mem[i][j] = true;
                    } else if i != 0 {
                        if nums[i - 1] as usize <= j {
                            mem[i][j] =
                                std::cmp::max(mem[i - 1][j], mem[i - 1][j - nums[i - 1] as usize]);
                        } else {
                            mem[i][j] = mem[i - 1][j];
                        }
                    }
                }
            }
            mem[alen][sum]
        }
    }
}

fn main() {
    unimplemented!();
}
