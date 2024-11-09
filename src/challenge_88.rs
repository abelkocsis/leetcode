pub fn run() {
    let mut vec1 = vec![4, 5, 6, 0, 0, 0];
    let m = 3;
    let mut vec2 = vec![1, 2, 3];
    let n = 3;
    Solution::merge(&mut vec1, m, &mut vec2, n);
    println!("{:?}", vec1);
}

struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = 0_usize;
        let mut j = 0_usize;
        let mut tmp_nums1: Vec<i32> = vec![*nums1.first().unwrap()];

        for k in 0..m as usize + n as usize {
            nums1[k] =
                if j == n as usize || (i < m as usize && *tmp_nums1.first().unwrap() <= nums2[j]) {
                    i += 1;
                    tmp_nums1.remove(0)
                } else {
                    j += 1;
                    nums2[j - 1]
                };
            if i < m as usize {
                tmp_nums1.push(nums1[k + 1]);
            }
        }
    }
}
