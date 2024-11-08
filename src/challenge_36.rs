pub fn run() {
    let board = vec![
        vec!['.', '6', '.', '.', '7', '.', '6', '.', '3'],
        vec!['.', '2', '.', '.', '.', '.', '.', '.', '7'],
        vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '7', '.', '.', '.'],
        vec!['.', '9', '4', '.', '.', '.', '8', '.', '.'],
        vec!['.', '.', '.', '2', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '6', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];
    println!("{}", Solution::is_valid_sudoku(board));
}

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // check rows
        let row_res = board.iter().all(|row| {
            let mut row_nums: Vec<&char> = row.iter().filter(|&&c| c != '.').collect();
            let row_num_count = row_nums.len();
            row_nums.sort();
            row_nums.dedup();
            row_num_count == row_nums.len()
        });

        if !row_res {
            return false;
        }

        // check columns
        let mut column_res = true;
        for j in 0..9 {
            let mut column: Vec<char> = Vec::new();

            board.iter().for_each(|row| {
                if row[j] != '.' {
                    if column.contains(&row[j]) {
                        column_res = false;
                    } else {
                        column.push(row[j]);
                    }
                }
            });

            if !column_res {
                break;
            }
        }
        if !column_res {
            return false;
        }

        // check sub_boxes
        for start_i in (0..9).step_by(3) {
            for start_j in (0..9).step_by(3) {
                let mut box_nums: Vec<char> = Vec::new();
                for i in start_i..start_i + 3 {
                    for j in start_j..start_j + 3 {
                        if board[i][j] != '.' {
                            if box_nums.contains(&board[i][j]) {
                                return false;
                            }
                            box_nums.push(board[i][j]);
                        }
                    }
                }
            }
        }

        true
    }
}
