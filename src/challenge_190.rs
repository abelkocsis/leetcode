pub fn run() {
    println!("{:?}", Solution::reverse_bits(43261596));
}

struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let binary_string = format!("{:032b}", x);
        let mut binary_vec: Vec<u8> = binary_string
            .chars()
            .map(|c| c.to_digit(2).unwrap() as u8)
            .collect();
        binary_vec.reverse();

        binary_vec
            .iter()
            .fold(0, |acc, &bit| (acc << 1) | bit as u32)
    }
}
