pub fn run() {
    println!("{:?}", Solution::three_sum(vec![0, 0, 0]));
}

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ns = nums.clone();
        ns.sort();

        let first_positive = ns
            .iter()
            .enumerate()
            .filter(|(_, &v)| v > 0)
            .map(|(i, _)| i)
            .min();

        let first_non_negative = ns
            .iter()
            .enumerate()
            .filter(|(_, &v)| v >= 0)
            .map(|(i, _)| i)
            .min();

        let mut j: usize;
        let mut k: usize;
        let mut sol: Vec<Vec<i32>> = Vec::new();

        for i in 0..=first_non_negative.unwrap_or(0).min(ns.len() - 3) {
            j = i + 1;
            while j < ns.len() - 1 {
                let sum_so_far = ns[i] + ns[j];

                k = match sum_so_far.cmp(&0) {
                    std::cmp::Ordering::Greater => ns.len(),
                    std::cmp::Ordering::Equal => first_non_negative.unwrap_or(ns.len()).max(j + 1),
                    std::cmp::Ordering::Less => first_positive.unwrap_or(ns.len()).max(j + 1),
                };

                while k < ns.len() {
                    let sum = sum_so_far + ns[k];
                    if sum == 0 {
                        sol.push(vec![ns[i], ns[j], ns[k]]);
                    }
                    if sum >= 0 {
                        break;
                    }
                    k += 1;
                }
                j += 1;
            }
        }

        sol.sort();
        sol.dedup();
        sol
    }
}
