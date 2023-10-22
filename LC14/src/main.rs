struct Solution {
}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut vec1 = strs;
        vec1.sort();
        let mut i = 0;
        while i < vec1[0].len() && i < vec1[vec1.len() - 1].len() {
            if vec1[0].chars().nth(i) != vec1[vec1.len() - 1].chars().nth(i) {
                break;
            }
            i += 1;
        }

        vec1[0][..i].to_string()
    }
}

fn main() {
    let vec1 = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("{}",Solution::longest_common_prefix(vec1));
}
