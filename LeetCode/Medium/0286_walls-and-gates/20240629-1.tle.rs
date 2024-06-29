use std::cmp::min;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let m = rooms.len();
        let n = rooms[0].len();

        let mut gates = vec![];
        for i in 0..m {
            for j in 0..n {
                if rooms[i][j] == 0 {
                    gates.push((i as i32, j as i32));
                }
            }
        }

        for gate in gates {
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();

            queue.push_back((gate.0, gate.1, 0));

            while let Some((i, j, d)) = queue.pop_front() {
                if i < 0
                    || i >= m as i32
                    || j < 0
                    || j >= n as i32
                    || visited.get(&(i, j)).is_some()
                    || rooms[i as usize][j as usize] == -1
                {
                    continue;
                }

                visited.insert((i, j));
                rooms[i as usize][j as usize] = min(rooms[i as usize][j as usize], d);

                queue.push_back((i + 1, j, d + 1));
                queue.push_back((i - 1, j, d + 1));
                queue.push_back((i, j + 1, d + 1));
                queue.push_back((i, j - 1, d + 1));
            }
        }
    }
}

struct Solution;
