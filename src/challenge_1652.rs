pub fn run() {
    println!("{:?}", Solution::decrypt(vec![2, 4, 9, 3], -2));
}

struct Solution {}

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![0; code.len()];
        if k == 0 {
            return res;
        }

        for i in 0..code.len() {
            res[i] = if k > 0 {
                (i + 1..=i + k as usize).map(|j| code[j % code.len()]).sum()
            } else {
                let mut start = i as i32 + k;
                if start < 0 {
                    start += code.len() as i32;
                }
                (start..start - k)
                    .map(|j| code[(j % code.len() as i32) as usize])
                    .sum()
            }
        }

        res
    }
}
