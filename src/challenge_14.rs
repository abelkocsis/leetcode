pub fn run() {
    println!(
        "{}",
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "do".to_string(),
            "dogcar".to_string()
        ])
    );
}

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs_vec: Vec<Vec<char>> = strs.iter().map(|str| str.chars().collect()).collect();
        let max_sol_len = strs_vec.iter().map(|str| str.len()).min().unwrap();

        for end in 0..max_sol_len {
            let c = strs_vec[0][end];
            if !strs_vec.iter().all(|str| str[end] == c) {
                return strs_vec[0][0..end].to_vec().iter().collect();
            }
        }
        strs_vec[0][0..max_sol_len].to_vec().iter().collect()
    }
}
