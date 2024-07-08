use std::collections::HashSet;
use std::io::{self};

struct ChessInfo {
    blocked: HashSet<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl ChessInfo {
    fn new(raw_blocked: Vec<Vec<char>>) -> Self {
        Self {
            blocked: board_to_blocked(raw_blocked),
            visited: HashSet::new(),
        }
    }

    fn solve(&mut self, row: i32) -> i32 {
        if row >= 8 {
            return 1;
        }

        let mut count = 0;

        for j in 0..8 {
            let pos = (row, j);
            if !self.is_placeable(pos) {
                continue;
            }

            self.visited.insert(pos);
            count += self.solve(row + 1);
            self.visited.remove(&pos);
        }

        count
    }

    fn is_placeable(&self, pos: (i32, i32)) -> bool {
        if self.blocked.contains(&pos) {
            return false;
        }

        for &visited_pos in &self.visited {
            if pos.0 == visited_pos.0 || pos.1 == visited_pos.1 {
                return false;
            }

            if (pos.0 - visited_pos.0).abs() == (pos.1 - visited_pos.1).abs() {
                return false;
            }
        }

        true
    }
}

fn board_to_blocked(board: Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == '*' {
                result.insert((i as i32, j as i32));
            }
        }
    }

    result
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut raw_blocked = vec![];

    for _ in 0..8 {
        io::stdin().read_line(&mut buf)?;
        raw_blocked.push(buf.trim().chars().collect::<Vec<char>>());
        buf.clear();
    }

    println!("{}", ChessInfo::new(raw_blocked).solve(0));

    Ok(())
}
