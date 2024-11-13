pub fn run() {
    println!(
        "{}",
        Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11)
    );
}

struct Solution {}

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut res = 0_i64;

        let mut j = 1_usize;
        let mut start_with_j = nums.len() - 1;
        let mut end_with_j = 0_usize;

        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        j = 0;

        for i in 0..sorted_nums.len() - 1 {
            j = start_with_j.max(i + 1).min(sorted_nums.len() - 1);
            while j - 1 > i && sorted_nums[i] + sorted_nums[j] >= lower {
                j -= 1;
            }
            while j < sorted_nums.len() && sorted_nums[i] + sorted_nums[j] < lower {
                j += 1;
            }
            start_with_j = j;

            if start_with_j == sorted_nums.len() {
                continue;
            }

            j = end_with_j.max(i + 1).min(sorted_nums.len() - 1);
            while j < sorted_nums.len() && sorted_nums[i] + sorted_nums[j] <= upper {
                j += 1;
            }
            while j < sorted_nums.len() && j > i && sorted_nums[i] + sorted_nums[j] > upper {
                j -= 1;
            }
            end_with_j = j;

            if start_with_j > end_with_j {
                continue;
            }

            if end_with_j == sorted_nums.len() || start_with_j == sorted_nums.len() {
                res += end_with_j as i64 - start_with_j as i64;
            } else {
                res += end_with_j as i64 - start_with_j as i64 + 1;
            }
        }

        res
    }
}
