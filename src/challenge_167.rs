pub fn run() {
    println!("{:?}", Solution::two_sum(vec![5, 25, 75], 100));
}

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            let res = Solution::binary_search(&numbers, target - numbers[i], i + 1, numbers.len());
            if let Some(j) = res {
                return vec![i as i32 + 1, j as i32 + 1];
            }
        }
        Vec::new()
    }

    pub fn binary_search(numbers: &[i32], value: i32, min: usize, max: usize) -> Option<usize> {
        if min == max {
            return None;
        }

        let j = (min + max) / 2;

        match numbers[j].cmp(&value) {
            std::cmp::Ordering::Equal => Some(j),
            std::cmp::Ordering::Less => Solution::binary_search(numbers, value, j + 1, max),
            std::cmp::Ordering::Greater => Solution::binary_search(numbers, value, min, j),
        }
    }
}
