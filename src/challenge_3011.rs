pub fn run() {
    println!("Res: {}", Solution::can_sort_array(vec![20, 16]));
}

struct Solution {}

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut previous_group_maxes: Vec<i32> = Vec::new();

        let mut nums_iter = nums.iter();

        let mut next = nums_iter.next();
        let mut val = *next.unwrap();
        let mut current_group_setbit = val.count_ones();
        let mut current_max = val;
        next = nums_iter.next();

        while next.is_some() {
            val = *next.unwrap();

            if val.count_ones() == current_group_setbit {
                if val > current_max {
                    current_max = val;
                }
            } else {
                previous_group_maxes.push(current_max);

                current_group_setbit = val.count_ones();
                current_max = val;
            }

            if previous_group_maxes.iter().any(|&n| n > val) {
                return false;
            }

            next = nums_iter.next();
        }

        true
    }
}
