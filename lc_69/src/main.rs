impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }
        let mut x: u64 = x as u64;
        let mut left = 1;
        let mut right = x;
        while left < right {
            let mid = left + (right - left) / 2;
            if mid > x / mid {
                right = mid;
            } else if mid < x / mid {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }
        right as i32 -1
    }
}

struct Solution;



fn main() {
    println!("Solution::my_sqrt(4) = {}", Solution::my_sqrt(4));
    println!("Solution::my_sqrt(48) = {}", Solution::my_sqrt(48));
    println!("Solution::my_sqrt(21474836478) = {}", Solution::my_sqrt(2147483647));
}
