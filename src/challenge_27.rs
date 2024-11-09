pub fn run() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    println!("{}", Solution::remove_element(&mut nums, val));
    println!("{:?}", nums);
}

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut place = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[place as usize] = nums[i];
                place += 1;
            }
        }
        place
    }
}
