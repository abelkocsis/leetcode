use std::usize;

pub fn run() {
    println!("{}", Solution::minimum_subarray_length(vec![1, 2, 3], 2));
}

struct Solution {}

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut min: i32 = i32::MAX;

        let k_bin_string = format!("{:032b}", k);
        let k_bin_vec: Vec<u8> = k_bin_string
            .chars()
            .map(|c| c.to_digit(2).unwrap() as u8)
            .collect();

        let mut last_tick_for_bit: Vec<i32> = vec![-1; k_bin_vec.len()];

        for (num_i, num) in nums.iter().enumerate() {
            let num_bin_string = format!("{:032b}", num);
            let num_bin_vec: Vec<u8> = num_bin_string
                .chars()
                .map(|c| c.to_digit(2).unwrap() as u8)
                .collect();

            num_bin_vec
                .iter()
                .enumerate()
                .filter(|&(_, &b)| b == 1)
                .for_each(|(i, &b)| {
                    if k_bin_vec[i] == 1 {
                        last_tick_for_bit[i] = num_i as i32;
                    } else {
                        last_tick_for_bit
                            .iter_mut()
                            .skip(i)
                            .for_each(|t| *t = num_i as i32);
                    }
                });
            let smallest_to_make_special = last_tick_for_bit
                .iter()
                .enumerate()
                .filter(|&(i, _)| k_bin_vec[i] == 1)
                .map(|(_, &n)| n)
                .min()
                .unwrap_or(num_i as i32);
            if smallest_to_make_special != -1 && num_i as i32 - smallest_to_make_special + 1 < min {
                min = num_i as i32 - smallest_to_make_special + 1;
            }
            if min == 1 {
                return 1;
            }
        }
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}
