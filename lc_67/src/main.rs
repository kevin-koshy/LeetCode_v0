use std::fmt::format;

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = 0;
        let char_width = std::cmp::max(a.len(), b.len());
        println!("char_width: {}", char_width);
        let mut a = format!("{:0>char_width$}", a);
        let mut b = format!("{:0>char_width$}", b);
        println!("a:{}",a);
        println!("b:{}", b);



    "".to_string()
    }
}


fn main() {
    println!("Hello, world!");
    let a = "111111".to_string();
    let b = "1".to_string();
    let result = Solution::add_binary(a, b);
    println!("{}", result);
}
