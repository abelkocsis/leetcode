use std::i32;

pub fn run() {
    let vec = vec![2, 3, 0, 1, 4];
    println!("{:?}", Solution::jump(vec));
}

struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut sol = vec![i32::MAX; nums.len()];

        sol[nums.len() - 1] = 0;
        for start in (0..nums.len()).rev() {
            if sol[start] == i32::MAX {
                continue;
            }
            for j in 0..start {
                if sol[j] > sol[start] + 1 && nums[j] >= start as i32 - j as i32 {
                    sol[j] = sol[start] + 1;
                }
            }
        }

        sol[0]
    }
}
