struct Solution {}
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        if start_fuel >= target {
            return 0;
        }
        let mut reach = start_fuel;
        let mut max_r = 0;
        for st in stations {
            if st[0] < reach {
                if reach - st[0] + st[1] > max_r {
                    max_r = reach - st[0] + st[1];
                }
            }
        }
        -1
    }
}
fn main() {
    println!("Hello, world!");
}
