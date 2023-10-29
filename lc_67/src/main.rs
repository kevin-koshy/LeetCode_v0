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

        let mut count = char_width;
        let mut result = String::new();
        while count > 0 {
            // let mut result = '0';
            if a.chars().nth(count -1).unwrap() == '1'{
                carry += 1;
            }
            if b.chars().nth(count -1).unwrap() == '1'{
                carry += 1;
            }
            if carry % 2 == 1 {
                result.push('1');
            }
            else {
                result.push('0');
            }
            carry /= 2;
            if carry == 1 {
                result.push('0');
            }
            count -= 1;
        }


        let result: String = result.chars().rev().collect();


    result.to_string()
    }
}


fn main() {
    println!("Hello, world!");
    let a = "111111".to_string();
    let b = "1".to_string();
    let result = Solution::add_binary(a, b);
    println!("{}", result);
}
