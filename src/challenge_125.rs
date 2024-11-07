pub fn run() {
    println!("{}", Solution::is_palindrome(" ".to_string()));
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        println!("{:?}", chars);

        let mut i = 0_usize;
        let mut j = chars.len().saturating_sub(1);

        while i < j {
            if chars[i] != chars[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
}
