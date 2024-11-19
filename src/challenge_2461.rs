use std::collections::HashSet;

pub fn run() {
    println!("{}", Solution::maximum_subarray_sum(vec![5, 3, 3, 1, 1], 3));
}

struct Solution {}

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut elems: HashSet<i32> = HashSet::new();
        let mut sum: i64 = 0;
        let k_u = k as usize;
        let mut max: i64 = 0;

        let mut start = 0_usize;

        for end in 0..nums.len() {
            if elems.len() == k_u {
                elems.remove(&nums[start]);
                sum -= nums[start] as i64;
                start += 1;
            }

            while elems.contains(&nums[end]) {
                elems.remove(&nums[start]);
                sum -= nums[start] as i64;
                start += 1;
            }
            elems.insert(nums[end]);
            sum += nums[end] as i64;

            if elems.len() == k_u {
                max = max.max(sum);
            }
        }

        max
    }
}
