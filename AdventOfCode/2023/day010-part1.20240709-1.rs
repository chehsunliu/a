use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::io::{self};

// ..F7.  ..45.
// .FJ|.  .236.
// SJ.L7  01.78
// |F--J  14567
// LJ...  23...

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.

pub struct Solver {
    tiles: Vec<Vec<char>>,
    visited: HashSet<(i32, i32)>,
    queue: VecDeque<((i32, i32), i32)>,
}

impl Solver {
    fn new(tiles: Vec<Vec<char>>) -> Self {
        Self {
            tiles,
            visited: HashSet::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn solve(&mut self) -> i32 {
        let mut ans = 0;
        let pos_s = self.find_s();
        self.queue.push_back((pos_s, 0));

        while let Some((pos, d)) = self.queue.pop_front() {
            if self.visited.contains(&pos) {
                continue;
            }

            self.visited.insert(pos);
            ans = max(ans, d);

            match self.tiles[pos.0 as usize][pos.1 as usize] {
                'S' => {
                    self.go_east(pos, d);
                    self.go_west(pos, d);
                    self.go_south(pos, d);
                    self.go_north(pos, d);
                }
                '|' => {
                    self.go_south(pos, d);
                    self.go_north(pos, d);
                }
                '-' => {
                    self.go_east(pos, d);
                    self.go_west(pos, d);
                }
                'L' => {
                    self.go_east(pos, d);
                    self.go_north(pos, d);
                }
                'J' => {
                    self.go_west(pos, d);
                    self.go_north(pos, d);
                }
                '7' => {
                    self.go_west(pos, d);
                    self.go_south(pos, d);
                }
                'F' => {
                    self.go_east(pos, d);
                    self.go_south(pos, d);
                }
                _ => unreachable!(),
            }
        }

        ans
    }

    fn go_east(&mut self, (i, j): (i32, i32), d: i32) {
        let (i, j) = (i, j + 1);
        if j >= self.tiles[0].len() as i32 {
            return;
        }

        match self.tiles[i as usize][j as usize] {
            '-' | 'J' | '7' => self.queue.push_back(((i, j), d + 1)),
            _ => {}
        }
    }

    fn go_west(&mut self, (i, j): (i32, i32), d: i32) {
        let (i, j) = (i, j - 1);
        if j < 0 {
            return;
        }

        match self.tiles[i as usize][j as usize] {
            '-' | 'L' | 'F' => self.queue.push_back(((i, j), d + 1)),
            _ => {}
        }
    }

    fn go_south(&mut self, (i, j): (i32, i32), d: i32) {
        let (i, j) = (i + 1, j);
        if i >= self.tiles.len() as i32 {
            return;
        }

        match self.tiles[i as usize][j as usize] {
            '|' | 'L' | 'J' => self.queue.push_back(((i, j), d + 1)),
            _ => {}
        }
    }

    fn go_north(&mut self, (i, j): (i32, i32), d: i32) {
        let (i, j) = (i - 1, j);
        if i < 0 {
            return;
        }

        match self.tiles[i as usize][j as usize] {
            '|' | 'F' | '7' => self.queue.push_back(((i, j), d + 1)),
            _ => {}
        }
    }

    fn find_s(&self) -> (i32, i32) {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                if self.tiles[i][j] == 'S' {
                    return (i as i32, j as i32);
                }
            }
        }

        unreachable!();
    }
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut tiles: Vec<Vec<char>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        tiles.push(buf.chars().collect());
        buf.clear();
    }

    println!("{}", Solver::new(tiles).solve());

    Ok(())
}
