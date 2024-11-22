pub fn run() {
    println!("{}", Solution::range_bitwise_and(2147483646, 2147483647));
}

struct Solution {}

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let bs = format!("{:032b}", left);
        let left_bits: Vec<u8> = bs.chars().map(|c| c.to_digit(2).unwrap() as u8).collect();

        let one_bits: Vec<usize> = left_bits
            .iter()
            .enumerate()
            .filter(|&(_, &b)| b == 1)
            .map(|(i, _)| i)
            .collect();

        let mut final_one_bits: Vec<usize> = Vec::new();

        for one_index in one_bits {
            let mut num = left_bits.clone();
            let mut i = one_index - 1;
            while num[i] == 1 {
                i -= 1;
            }
            num[i] = 1;
            i += 1;
            while i < num.len() {
                num[i] = 0;
                i += 1;
            }
            let val = num.iter().fold(0, |acc, &bit| (acc << 1) | bit as i64);
            if val > right as i64 {
                final_one_bits.push(one_index);
            }
        }

        let mut sol: Vec<u8> = vec![0; 32];
        for one_bit in final_one_bits {
            sol[one_bit] = 1;
        }

        sol.iter().fold(0, |acc, &bit| (acc << 1) | bit as i32)
    }
}
