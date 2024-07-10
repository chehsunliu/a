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

pub struct Maze {
    tiles: Vec<Vec<char>>,
    loop_tiles: HashSet<(i32, i32)>,
    outer_tiles: HashSet<(i32, i32)>,
}

impl Maze {
    pub fn new(tiles: Vec<Vec<char>>) -> Self {
        Self {
            tiles,
            loop_tiles: HashSet::new(),
            outer_tiles: HashSet::new(),
        }
    }

    pub fn print(&self) {
        let mut s = "".to_string();
        for i in 0..self.tiles.len() as i32 {
            for j in 0..self.tiles[0].len() as i32 {
                if self.loop_tiles.contains(&(i, j)) {
                    s.push(self.tiles[i as usize][j as usize]);
                    // s.push('x');
                } else if self.outer_tiles.contains(&(i, j)) {
                    s.push(' ');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        println!("{s}");
    }

    pub fn identify_outer_tiles(&mut self) -> i32 {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        for i in 0..self.tiles.len() {
            queue.push_back((i as i32, 0));
            queue.push_back((i as i32, self.tiles[0].len() as i32 - 1));
        }
        for j in 0..self.tiles[0].len() {
            queue.push_back((0, j as i32));
            queue.push_back((self.tiles.len() as i32 - 1, j as i32));
        }

        while let Some(pos) = queue.pop_back() {
            if self.loop_tiles.contains(&pos) || self.outer_tiles.contains(&pos) {
                continue;
            }

            self.outer_tiles.insert(pos);

            let next_poses = [
                (pos.0 - 1, pos.1),
                (pos.0 + 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
            ];
            for next_pos in next_poses {
                if next_pos.0 >= 0
                    && next_pos.0 < self.tiles.len() as i32
                    && next_pos.1 >= 0
                    && next_pos.1 < self.tiles[0].len() as i32
                {
                    queue.push_back(next_pos);
                }
            }
        }

        self.tiles.len() as i32 * self.tiles[0].len() as i32
            - self.loop_tiles.len() as i32
            - self.outer_tiles.len() as i32
    }

    pub fn identify_loop_tiles(&mut self) -> i32 {
        let mut ans = 0;
        let pos_s = self.find_s();
        let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();

        queue.push_back((pos_s, 0));

        while let Some((pos, d)) = queue.pop_front() {
            if self.loop_tiles.contains(&pos) {
                continue;
            }

            self.loop_tiles.insert(pos);
            ans = max(ans, d);

            match self.tiles[pos.0 as usize][pos.1 as usize] {
                'S' => {
                    self.go_east(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                    self.go_west(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                    self.go_south(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                    self.go_north(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                }
                '|' => {
                    self.go_south(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                    self.go_north(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                }
                '-' => {
                    self.go_east(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                    self.go_west(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                }
                'L' => {
                    self.go_east(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                    self.go_north(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                }
                'J' => {
                    self.go_west(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                    self.go_north(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                }
                '7' => {
                    self.go_west(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                    self.go_south(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                }
                'F' => {
                    self.go_east(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                    self.go_south(pos)
                        .map(|new_pos| queue.push_back((new_pos, d + 1)));
                }
                _ => unreachable!(),
            }
        }

        ans
    }

    fn go_east(&self, (i, j): (i32, i32)) -> Option<(i32, i32)> {
        let (i, j) = (i, j + 1);
        if j >= self.tiles[0].len() as i32 {
            return None;
        }

        match self.tiles[i as usize][j as usize] {
            '-' | 'J' | '7' => Some((i, j)),
            _ => None,
        }
    }

    fn go_west(&self, (i, j): (i32, i32)) -> Option<(i32, i32)> {
        let (i, j) = (i, j - 1);
        if j < 0 {
            return None;
        }

        match self.tiles[i as usize][j as usize] {
            '-' | 'L' | 'F' => Some((i, j)),
            _ => None,
        }
    }

    fn go_south(&self, (i, j): (i32, i32)) -> Option<(i32, i32)> {
        let (i, j) = (i + 1, j);
        if i >= self.tiles.len() as i32 {
            return None;
        }

        match self.tiles[i as usize][j as usize] {
            '|' | 'L' | 'J' => Some((i, j)),
            _ => None,
        }
    }

    fn go_north(&self, (i, j): (i32, i32)) -> Option<(i32, i32)> {
        let (i, j) = (i - 1, j);
        if i < 0 {
            return None;
        }

        match self.tiles[i as usize][j as usize] {
            '|' | 'F' | '7' => Some((i, j)),
            _ => None,
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

    let mut maze = Maze::new(tiles);
    println!("{}", maze.identify_loop_tiles());
    println!("{}", maze.identify_outer_tiles());
    maze.print();

    Ok(())
}
