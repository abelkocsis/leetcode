pub fn run() {
    println!("{}", Solution::min_changes("0000".to_string()));
}

struct Solution {}

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let char_vec: Vec<char> = s.chars().collect();
        let mut res = 0;
        for (i, &c) in char_vec.iter().enumerate() {
            if i % 2 != 0 && c != char_vec[i - 1] {
                res += 1;
            }
        }
        res
    }
}
