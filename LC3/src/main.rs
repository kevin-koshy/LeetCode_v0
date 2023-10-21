struct Solution {

}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
    let collection: Vec<char> = vec![s.chars().next().unwrap()];
        for (index, c ) in s.chars().enumerate() {
            println!("c = {}", c);
        }
        3
    }
}


fn main() {
    let b = Solution::length_of_longest_substring(String::from("pwwwkew"));
    println!("b = {}", b);
}
