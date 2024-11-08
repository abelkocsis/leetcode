pub fn run() {
    let vec = vec![0, 1, 1, 3];
    let max_bit = 2;
    println!("{:?}", Solution::get_maximum_xor(vec, max_bit));
}

struct Solution {}

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let max_bit = maximum_bit as usize;
        let mut xored: Vec<bool> = vec![false; max_bit];

        let mut binary_nums: Vec<Vec<u8>> = nums
            .iter()
            .map(|n| {
                let binary_string = format!("{:032b}", n);
                let binary_vec: Vec<u8> = binary_string
                    .chars()
                    .map(|c| c.to_digit(2).unwrap() as u8)
                    .collect();
                binary_vec[binary_vec.len() - max_bit..].to_vec()
            })
            .collect();

        binary_nums.iter().for_each(|b_v| {
            b_v.iter()
                .enumerate()
                .for_each(|(i, b)| xored[i] ^= *b == 1);
        });

        let mut res: Vec<i32> = Vec::new();
        while !binary_nums.is_empty() {
            res.push(Solution::calculate_k(&xored, max_bit));

            let deleted = binary_nums.pop().unwrap();
            deleted
                .iter()
                .enumerate()
                .for_each(|(i, b)| xored[i] ^= *b == 1);
        }

        res
    }

    pub fn calculate_k(xored: &[bool], max_bit: usize) -> i32 {
        let mut res_binary: Vec<u8> = Vec::new();
        xored.iter().enumerate().for_each(|(i, b)| {
            if i > max_bit {
                res_binary.push(if *b { 1 } else { 0 });
            } else {
                res_binary.push(if b ^ true { 1 } else { 0 });
            }
        });

        res_binary
            .iter()
            .fold(0, |acc, &bit| (acc << 1) | bit as i32)
    }
}
