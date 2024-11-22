pub fn run() {
    println!("{:?}", Solution::generate_parenthesis(3));
}

struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let r = Self::generate_parenthesis_c(n as usize, 0, 0);
        r.iter().map(|v| v.iter().collect()).collect()
    }

    pub fn generate_parenthesis_c(n: usize, opened: usize, closed: usize) -> Vec<Vec<char>> {
        let mut res: Vec<Vec<char>> = Vec::new();
        if opened == n {
            res.push(vec![')'; n - closed]);
        } else {
            // could always open a new one
            let mut subres_if_opened = Self::generate_parenthesis_c(n, opened + 1, closed);
            for subr in subres_if_opened.iter_mut() {
                subr.insert(0, '(');
            }
            res.extend(subres_if_opened);

            // or could close one if possible
            if opened > closed {
                let mut subres_if_closed = Self::generate_parenthesis_c(n, opened, closed + 1);
                for subr in subres_if_closed.iter_mut() {
                    subr.insert(0, ')');
                }
                res.extend(subres_if_closed);
            }
        }

        res
    }
}
