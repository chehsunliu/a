use std::collections::HashSet;

type Board = Vec<Vec<char>>;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut cells = find_empty_cells(board);
        solve(board, &mut cells, 0);
    }
}

fn solve(board: &mut Board, cells: &mut Vec<(usize, usize)>, index: usize) -> bool {
    if index >= cells.len() {
        return true;
    }

    let cell = cells[index];
    let available_numbers = find_available_numbers(board, cell);

    for number in available_numbers {
        board[cell.0][cell.1] = number;
        if solve(board, cells, index + 1) {
            return true;
        }
        board[cell.0][cell.1] = '.';
    }

    false
}

fn find_empty_cells(board: &Board) -> Vec<(usize, usize)> {
    let mut cells = Vec::new();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == '.' {
                cells.push((i, j));
            }
        }
    }
    cells
}

fn find_available_numbers(board: &Board, pos: (usize, usize)) -> Vec<char> {
    let mut numbers = ('1'..='9').collect::<HashSet<char>>();

    for j in 0..9 {
        numbers.remove(&board[pos.0][j]);
    }

    for i in 0..9 {
        numbers.remove(&board[i][pos.1]);
    }

    let offset_i = pos.0 / 3;
    let offset_j = pos.1 / 3;
    for i in 0..3 {
        for j in 0..3 {
            numbers.remove(&board[offset_i * 3 + i][offset_j * 3 + j]);
        }
    }

    let mut numbers: Vec<char> = numbers.into_iter().collect();
    numbers.sort();
    numbers
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
    }
}
