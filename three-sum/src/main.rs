// https://leetcode.com/problems/3sum/

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn two_sum(n: &Vec<i32>, sum: i32, taken_pos: usize) -> HashSet<Vec<i32>> {
        let mut set: HashSet<i32> = HashSet::new();
        let mut ans: HashSet<Vec<i32>> = HashSet::new();
        for (pos, ele) in n.iter().enumerate() {
            if pos != taken_pos {
                if set.contains(&(sum - ele)) {
                    ans.insert(vec![*ele, sum - ele]);
                } else {
                    set.insert(*ele);
                }
            }
        }
        ans
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut mem: HashSet<Vec<i32>> = HashSet::new();
        let mut pos = 0usize;

        for ele in nums.clone() {
            let temp = Solution::two_sum(&nums, -ele, pos);
            for k in temp {
                let mut t = vec![ele];
                t.extend(k);
                t.sort();
                mem.insert(t);
            }
            pos += 1;
        }
        mem.into_iter().collect::<Vec<Vec<i32>>>()
    }
}

fn main() {
    unimplemented!();
}
