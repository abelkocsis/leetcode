pub fn run() {
    println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
}

struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = digits.clone();
        let mut i = digits.len() as i32 - 1;
        while i >= 0 {
            res[i as usize] += 1;
            if res[i as usize] == 10 {
                res[i as usize] = 0;
                i -= 1;
            } else {
                break;
            }
        }
        if res[0] == 0 {
            res.insert(0, 1);
        }
        res
    }
}
