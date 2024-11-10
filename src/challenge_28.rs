pub fn run() {
    println!("{}", Solution::str_str("a".to_string(), "a".to_string()));
}

struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        /*if let Some(i) = haystack.find(&needle) {
            i as i32
        } else {
            -1
        }*/
        if haystack.len() < needle.len() {
            return -1;
        }
        for start in 0..=haystack.len() - needle.len() {
            if haystack[start..start + needle.len()].eq(&needle) {
                return start as i32;
            }
        }
        -1
    }
}
