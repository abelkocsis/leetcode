pub fn run() {
    println!("{}", Solution::prime_sub_operation(vec![5, 8, 3]));
}

struct Solution {}

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let primes: Vec<i32> = (2..=1000).filter(|&n| (2..n).all(|m| n % m != 0)).collect();
        //println!("{:?}", primes);

        let mut prev = 0;
        let mut current;
        for num in nums {
            current = Solution::substract(num, &primes, prev);
            if current <= prev {
                return false;
            }
            prev = current;
            // println!("Replacing {} with {}", num, current);
        }

        true
    }

    pub fn substract(num: i32, primes: &[i32], prev: i32) -> i32 {
        let biggest = primes
            .iter()
            .take_while(|&&p| p < num)
            .filter(|&&p| num - p > prev)
            .max();

        if let Some(&biggest_p) = biggest {
            num - biggest_p
        } else {
            num
        }
    }
}
