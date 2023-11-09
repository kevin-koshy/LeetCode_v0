struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut x:Vec<char> = s.chars().collect();
        let mut y:Vec<char> = t.chars().collect();
        x.sort();
        y.sort();
        // for i in 0..s.len() {
        //     if x[i] != y[i] {
        //         return false;
        //     }
        // }
        for char in x.iter(){
            y.binary_search(&char).is_ok();
            
        }
        true
    }
}

fn main() {
    println!("{}", Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
    println!("Hello, world!");
}
