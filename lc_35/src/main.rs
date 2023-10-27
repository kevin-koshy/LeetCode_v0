struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for i in 0..nums.len() {
            if nums[i] >= target {
                return i as i32;
            }
            else {
                if i == nums.len() - 1 {
                    return i as i32 + 1;
                }
            }
        }
        -1
        }
}

fn main() {
    let vec = vec![1, 3, 5, 6];
    println!("{}", Solution::search_insert(vec, 676));
}