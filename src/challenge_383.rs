use std::collections::HashMap;

pub fn run() {
    println!(
        "{}",
        Solution::can_construct("aa".to_string(), "aab".to_string())
    );
}

struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut characters: HashMap<char, i32> = HashMap::new();

        magazine.chars().for_each(|c| {
            if let Some(&old_val) = characters.get(&c) {
                characters.insert(c, old_val + 1);
            } else {
                characters.insert(c, 1);
            }
        });

        ransom_note.chars().for_each(|c| {
            characters.insert(c, characters.get(&c).unwrap_or(&0) - 1);
        });

        characters.iter().all(|(_, &v)| v >= 0)
    }
}
