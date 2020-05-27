struct Solution {}
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut temp_val = i32::min_value();
        let mut temp_len = 0;
        for ele in nums {
            if ele > temp_val {
                temp_len += 1;
            } else {
                temp_len = 1;
            }
            temp_val = ele;
            if temp_len > ans {
                ans = temp_len;
            }
        }
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
