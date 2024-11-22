pub fn run() {
    println!(
        "{}",
        Solution::add_binary("101111".to_string(), "10".to_string())
    );
}

struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a: Vec<char> = a.chars().rev().collect();
        let b: Vec<char> = b.chars().rev().collect();

        let mut rem = 0;

        let mut sol: Vec<char> = Vec::new();

        for i in 0..a.len().max(b.len()) {
            if i >= a.len() {
                match (rem, b[i]) {
                    (0, _) => sol.push(b[i]),
                    (1, '1') => sol.push('0'),
                    (1, '0') => {
                        sol.push('1');
                        rem = 0;
                    }
                    _ => panic!(),
                }
            } else if i >= b.len() {
                match (rem, a[i]) {
                    (0, _) => sol.push(a[i]),
                    (1, '1') => sol.push('0'),
                    (1, '0') => {
                        sol.push('1');
                        rem = 0;
                    }
                    _ => panic!(),
                }
            } else {
                let sum = rem + (a[i] as i32 - '0' as i32) + (b[i] as i32 - '0' as i32);
                match sum {
                    0 => {
                        sol.push('0');
                        rem = 0;
                    }
                    1 => {
                        sol.push('1');
                        rem = 0;
                    }
                    2 => {
                        sol.push('0');
                        rem = 1;
                    }
                    3 => {
                        sol.push('1');
                        rem = 1;
                    }
                    _ => panic!(),
                }
            }
        }

        if rem == 1 {
            sol.push('1');
        }

        sol.iter().rev().collect()
    }
}
