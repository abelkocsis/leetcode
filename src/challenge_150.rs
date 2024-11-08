pub fn run() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;

        for i in 0..height.len() - 1 {
            let i_height = height[i];
            if i_height == 0 {
                continue;
            }

            height
                .iter()
                .enumerate()
                .skip(((max + i as i32 * i_height) / i_height) as usize)
                .for_each(|(j, val)| {
                    let min = (j - i) as i32 * i_height.min(*val);
                    if min > max {
                        max = min;
                    }
                });
        }

        max
    }
}
