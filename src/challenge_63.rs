pub fn run() {
    let obstacle_grid = vec![
        vec![0, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0],
    ];
    println!("{}", Solution::unique_paths_with_obstacles(obstacle_grid));
}

struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut all_paths: Vec<Vec<i32>> =
            vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];
        let mut round_start: Vec<(usize, usize)> = Vec::new();
        if obstacle_grid[0][0] == 0 {
            all_paths[0][0] = 1;
            round_start.push((0, 0));
        }

        Self::pathfinder(&obstacle_grid, &mut all_paths, &mut round_start);
        for v in all_paths.iter() {
            println!("{:?}", v);
        }
        *all_paths.last().unwrap().last().unwrap()
    }

    pub fn pathfinder(
        obstacle_grid: &[Vec<i32>],
        all_paths: &mut [Vec<i32>],
        round_start: &mut [(usize, usize)],
    ) {
        let mut next_round_start: Vec<(usize, usize)> = Vec::new();
        if round_start.is_empty() {
            return;
        }
        for &(i, j) in round_start.iter() {
            if i + 1 < obstacle_grid.len() && obstacle_grid[i + 1][j] == 0 {
                all_paths[i + 1][j] += all_paths[i][j];
                next_round_start.push((i + 1, j));
            }
            if j + 1 < obstacle_grid[0].len() && obstacle_grid[i][j + 1] == 0 {
                all_paths[i][j + 1] += all_paths[i][j];
                next_round_start.push((i, j + 1));
            }
        }
        next_round_start.sort();
        next_round_start.dedup();
        Self::pathfinder(obstacle_grid, all_paths, &mut next_round_start);
    }
}
