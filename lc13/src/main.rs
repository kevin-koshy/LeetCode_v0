use log::debug;

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_number_vector = s.chars().collect::<Vec<char>>();
        let mut ans = 0;
        let mut index2;
        let vec1 = Vec::from([1, 5, 10, 50, 100, 500, 1000]);
        let vec2 = Vec::from(["I", "V", "X", "L", "C", "D", "M"]);

        for i in 0..roman_number_vector.len() {
            println!("{}",i);
            let index1 = vec2.iter().position(|&x | x == roman_number_vector[i].to_string()).unwrap();
            if i == roman_number_vector.len() - 1 {
                index2 = 0;
            }
            else {
                index2 = vec2.iter().position(|&x | x == roman_number_vector[i+1].to_string()).unwrap();
            }
            println!("{} {}", vec1[index1], vec1[index2]);
            if vec1[index1] < vec1[index2] {
                ans -= vec1[index1];
            }
            else {
                ans += vec1[index1];
            }
        }
        ans
    }
}


fn main() {
    let y = Solution::roman_to_int(String::from("IX"));
    println!("{}", y);
}
