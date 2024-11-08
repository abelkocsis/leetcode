pub fn run() {
    println!(
        "{}",
        Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string())
    );
}

struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut i = 0_usize;
        let mut j = 0_usize;

        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();

        while j < t_vec.len() {
            if s_vec[i] == t_vec[j] {
                i += 1;
            }
            j += 1;

            if i == s_vec.len() {
                return true;
            }
        }
        false
    }
}
