pub fn run() {
    println!(
        "{}",
        Solution::length_of_last_word("Hello World".to_string())
    );
}

struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split(' ').filter(|w| !w.is_empty()).last().unwrap().len() as i32
    }
}
