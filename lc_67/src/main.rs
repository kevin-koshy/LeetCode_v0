struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a.chars().rev();
        let mut b = b.chars().rev();
        let mut carry = 0;
        let mut result = String::new();
        while a.len() > 0 || b.len() > 0 || carry != 0 {
            let mut sum = carry;
            if a.len() > 0 {

            }
        }
        result.chars().rev().collect()

    }
}


fn main() {
    println!("Hello, world!");
    let a = "11".to_string();
    let b = "1".to_string();
    let result = Solution::add_binary(a, b);
    println!("{}", result);
}
