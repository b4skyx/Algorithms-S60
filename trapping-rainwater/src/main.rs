struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 0 {
            return 0;
        }
        let max_height = height.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
        let mut front = 0;
        let mut end = height.len() - 1;
        let mut max_f = i32::min_value();
        let mut max_e = i32::min_value();
        let mut res = 0;
        while front != end {
            if front < max_height {
                if height[front] >= max_f {
                    max_f = height[front];
                } else {
                    res += max_f - height[front];
                }
                front += 1;
            }
            if end > max_height {
                if height[end] >= max_e {
                    max_e = height[end];
                } else {
                    res += max_e - height[end];
                }
                end -= 1;
            }
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
