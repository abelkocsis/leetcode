pub fn run() {
    println!("{}", Solution::longest_palindrome("abcba".to_string()));
}

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();

        // dp[i][j] true exactly if chars[i..=j] is a substring
        let mut dp: Vec<Vec<bool>> = vec![vec![false; chars.len()]; chars.len()];

        let mut max_len = 0_usize;
        let mut max_cordinates = (0_usize, 0_usize);

        // fill up dp
        for j in 0..chars.len() {
            for i in 0..=j {
                if i == j {
                    dp[i][j] = true;
                } else if j - i == 1 || j - i == 2 {
                    dp[i][j] = chars[i] == chars[j];
                } else {
                    dp[i][j] = dp[i + 1][j - 1] && chars[i] == chars[j];
                }

                if dp[i][j] && j - i + 1 > max_len {
                    max_len = j - i + 1;
                    max_cordinates = (i, j);
                }
            }
        }

        //println!("{:?}", dp);

        chars[max_cordinates.0..=max_cordinates.1].iter().collect()
    }
}
