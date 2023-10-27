struct  Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut parts = s.split_whitespace().collect::<Vec<&str>>();
        // println!("parts: {:?}", parts.len());
        parts.last().unwrap().len() as i32
    }
}

fn main() {
    let s = String::from("   fly me   to   the moon  ");
    println!("{}", Solution::length_of_last_word(s));
}
