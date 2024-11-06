pub fn run() {
    let grid = vec![
        vec![65, 200, 263, 220, 91, 183, 2, 187, 175, 61, 225, 120, 39],
        vec![111, 242, 294, 31, 241, 90, 145, 25, 262, 214, 145, 71, 294],
        vec![152, 25, 240, 69, 279, 238, 222, 9, 137, 277, 8, 143, 143],
        vec![189, 31, 86, 250, 20, 63, 188, 209, 75, 22, 127, 272, 110],
        vec![122, 94, 298, 25, 90, 169, 68, 3, 208, 274, 202, 135, 275],
        vec![205, 20, 171, 90, 70, 272, 280, 138, 142, 151, 80, 122, 130],
        vec![
            284, 272, 271, 269, 265, 134, 185, 243, 247, 50, 283, 20, 232,
        ],
        vec![
            266, 236, 265, 234, 249, 62, 98, 130, 122, 226, 285, 168, 204,
        ],
        vec![231, 24, 256, 101, 142, 28, 268, 82, 111, 63, 115, 13, 144],
        vec![277, 277, 31, 144, 49, 132, 28, 138, 133, 29, 286, 45, 93],
        vec![163, 96, 25, 9, 3, 159, 148, 59, 25, 81, 233, 127, 12],
        vec![127, 38, 31, 209, 300, 256, 15, 43, 74, 64, 73, 141, 200],
    ];
    println!("{}", Solution::max_moves(grid));
}

struct Solution {}

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid.first().unwrap().len();

        let mut res: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for j in (0..n).rev() {
            for i in 0..m {
                Self::check_step(&grid, &mut res, m, i, j);
            }
        }

        *res.iter()
            .filter_map(|row| row.first()) // Get the first element of each row (if it exists)
            .max()
            .unwrap()
    }

    pub fn check_step(grid: &[Vec<i32>], res: &mut [Vec<i32>], m: usize, i: usize, j: usize) {
        if j == 0 {
            return;
        }
        let prev_j = j - 1;

        for prev_i in i.checked_sub(1).unwrap_or(i)..=(i + 1).min(m - 1) {
            if grid[prev_i][prev_j] < grid[i][j] {
                let value = res[i][j] + 1;
                if res[prev_i][prev_j] < value {
                    res[prev_i][prev_j] = value;
                }
            }
        }
    }
}
