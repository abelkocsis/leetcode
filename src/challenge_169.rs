use std::collections::HashMap;

pub fn run() {
    println!(
        "{:?}",
        Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2])
    );
}

struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut elems: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(|&n| {
            if elems.contains_key(&n) {
                elems.insert(n, elems.get(&n).unwrap() + 1);
            } else {
                elems.insert(n, 1);
            }
        });

        *elems.iter().max_by_key(|&(_, &v)| v).unwrap().0
    }
}
