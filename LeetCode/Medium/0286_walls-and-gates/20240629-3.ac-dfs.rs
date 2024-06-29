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
            dfs(gate, 0, rooms);
        }
    }
}

fn dfs((i, j): (i32, i32), d: i32, rooms: &mut Vec<Vec<i32>>) {
    let m = rooms.len() as i32;
    let n = rooms[0].len() as i32;

    if i < 0 || i >= m || j < 0 || j >= n || rooms[i as usize][j as usize] == -1 {
        return;
    }

    if d == 0 || d < rooms[i as usize][j as usize] {
        rooms[i as usize][j as usize] = d;
        dfs((i + 1, j), d + 1, rooms);
        dfs((i - 1, j), d + 1, rooms);
        dfs((i, j + 1), d + 1, rooms);
        dfs((i, j - 1), d + 1, rooms);
    }
}

struct Solution;
