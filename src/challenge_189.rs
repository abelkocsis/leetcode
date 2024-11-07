pub fn run() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    Solution::rotate(&mut vec, k);
    println!("{:?}", vec);
}

struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut rotated: usize = 0;
        let mut round_start = 0;
        let mut i: usize = 0;
        let mut to_place = nums[i];
        let mut tmp;

        while rotated < nums.len() {
            let rotated_i = (i + k as usize) % nums.len();
            tmp = nums[rotated_i];
            nums[rotated_i] = to_place;
            rotated += 1;
            to_place = tmp;
            i = (i + k as usize) % nums.len();
            if i == round_start {
                i += 1;
                round_start = i;
                if i < nums.len() {
                    to_place = nums[i];
                }
            }
        }
    }
}
