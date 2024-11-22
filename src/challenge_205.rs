use std::collections::HashMap;

pub fn run() {
    println!(
        "{}",
        Solution::is_isomorphic("badc".to_string(), "baba".to_string())
    );
}

struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut replacements: HashMap<char, char> = HashMap::new();
        let mut to: Vec<char> = Vec::new();

        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();
        for (i, &c) in s_vec.iter().enumerate() {
            if let Some(&r) = replacements.get(&c) {
                if r != t_vec[i] {
                    return false;
                }
            } else {
                if to.contains(&t_vec[i]) {
                    return false;
                }
                replacements.insert(c, t_vec[i]);
                to.push(t_vec[i]);
            }
        }
        true
    }
}
