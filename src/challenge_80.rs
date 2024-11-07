pub fn run() {
    let mut vec = vec![1, 1, 1, 2, 2, 3];
    println!("{}", Solution::remove_duplicates(&mut vec));
    println!("{:?}", vec);
}

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut val = 0;
        let mut count: u8 = 0;

        while i < nums.len() {
            if nums[i] == val {
                if count < 2 {
                    nums[j] = val;
                    j += 1;
                    count += 1;
                }
            } else {
                val = nums[i];
                count = 1;
                nums[j] = val;
                j += 1;
            }
            i += 1
        }
        j as i32
    }
}
