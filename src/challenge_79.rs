use std::i32;

pub fn run() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    println!("{}", Solution::exist(board, "ABCCED".to_string()));
}

struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut starts: Vec<(usize, usize)> = Vec::new();
        let w: Vec<char> = word.chars().collect();

        for (i, v) in board.iter().enumerate() {
            for (j, &c) in v.iter().enumerate() {
                if c == w[0] {
                    starts.push((i, j));
                }
            }
        }

        let mut mark = vec![vec![false; board[0].len()]; board.len()];

        starts
            .iter()
            .any(|(i, j)| Solution::calculate(&board, &w[1..], &mut mark, *i, *j))
    }

    fn calculate(
        board: &Vec<Vec<char>>,
        word: &[char],
        mark: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
    ) -> bool {
        if word.is_empty() {
            return true;
        }
        mark[i][j] = true;

        if i > 0 && board[i - 1][j] == word[0] && !mark[i - 1][j] {
            if Solution::calculate(board, &word[1..], mark, i - 1, j) {
                return true;
            }
        }
        if i < board.len() - 1 && board[i + 1][j] == word[0] && !mark[i + 1][j] {
            if Solution::calculate(board, &word[1..], mark, i + 1, j) {
                return true;
            }
        }
        if j > 0 && board[i][j - 1] == word[0] && !mark[i][j - 1] {
            if Solution::calculate(board, &word[1..], mark, i, j - 1) {
                return true;
            }
        }
        if j < board[0].len() - 1 && board[i][j + 1] == word[0] && !mark[i][j + 1] {
            if Solution::calculate(board, &word[1..], mark, i, j + 1) {
                return true;
            }
        }
        mark[i][j] = false;
        false
    }
}
