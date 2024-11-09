pub fn run() {
    println!("{}", Solution::min_end(3, 4));
}

struct Solution {}

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let x_bitstring = format!("{:032b}", x);
        let mut x_bits = vec![0; 32];
        x_bits.extend(x_bitstring.chars().map(|c| c.to_digit(2).unwrap() as u8));

        // 1: true
        // 0: false
        // to be determined: None
        let result_fixed: Vec<Option<bool>> = x_bits
            .iter()
            .map(|&b| if b == 1 { Some(true) } else { None })
            .collect();

        let n_bitstring = format!("{:032b}", n - 1);
        let n_bits: Vec<u8> = n_bitstring
            .chars()
            .map(|c| c.to_digit(2).unwrap() as u8)
            .collect();

        let mut i: i32 = n_bits.len() as i32 - 1;

        let mut result: Vec<u8> = vec![0; 64];
        for j in (0..result_fixed.len()).rev() {
            if let Some(true) = result_fixed[j] {
                result[j] = 1u8;
            } else {
                result[j] = if i >= 0 { n_bits[i as usize] } else { 0 };
                i -= 1;
            }
        }

        result.iter().fold(0, |acc, &bit| (acc << 1) | bit as i64)
    }
}
