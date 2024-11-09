pub fn run() {
    let mut nums = vec![1, 1, 2];
    println!("{}", Solution::remove_duplicates(&mut nums));
    println!("{:?}", nums);
}

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut place = 1_usize;
        let mut prev = nums[0];
        for i in 1..nums.len() {
            if nums[i] != prev {
                nums[place] = nums[i];
                place += 1;
                prev = nums[i]
            }
        }
        place as i32
    }
}
