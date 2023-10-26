struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // nums.retain(|&x| x != val);
    // nums.len() as i32
        
        let mut i = 0;
        let mut j = 0;
        while  j < nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
            i as i32
        }
    }


fn main() {
    let mut nums = vec![0,1,2,2,3,0,4,2];
    // let mut nums = vec![3,2,2,3];
    println!("{}", Solution::remove_element(&mut nums, 2));
    println!("{:?}", nums);
}
