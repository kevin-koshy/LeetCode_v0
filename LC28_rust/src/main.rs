struct Solution;
impl Solution {
    pub fn str_str(_haystack: String, _needle: String) -> i32 {
        if &_haystack.len() < &_needle.len() {
            return -1;
        }
        for i in 0.._haystack.len() - _needle.len() + 1 {
            if _haystack[i..i + _needle.len()] == _needle {
                return i as i32;
            }
        }
        return -1;

    }
}

fn main() {
    let string1 = String::from("xsadbutsad");
    let string2 = String::from("sadsdafasdfasdfas");
    println!("{}", Solution::str_str(string1, string2));
}
