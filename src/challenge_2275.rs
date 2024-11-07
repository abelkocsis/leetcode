pub fn run() {
    println!(
        "{}",
        Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14])
    );
}

struct Solution {}

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut sum_of_numbers_with_bit: Vec<u32> = vec![0; 32];
        for c in candidates {
            let binary_string = format!("{:032b}", c);
            let binary_vec: Vec<u8> = binary_string
                .chars()
                .map(|c| c.to_digit(2).unwrap() as u8)
                .collect();
            binary_vec
                .iter()
                .enumerate()
                .for_each(|(i, v)| sum_of_numbers_with_bit[i] += *v as u32);
        }

        *sum_of_numbers_with_bit.iter().max().unwrap() as i32
    }
}
