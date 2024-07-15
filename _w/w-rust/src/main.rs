use std::collections::{HashSet, VecDeque};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut tiles: Vec<Vec<char>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        tiles.push(buf.trim().chars().collect());
        buf.clear();
    }

    Solver::new(tiles).solve();

    Ok(())
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

// .F----7F7F7F7F-7....
// .|F--7||||||||FJ....
// .||.FJ||||||||L7....
// FJL7L7LJLJ||LJ.L-7..
// L--J.L7...LJS7F-7L7.
// ....F-J..F7FJ|L7L7L7
// ....L7.F7||L7|.L7L7|
// .....|FJLJ|FJ|F7|.LJ
// ....FJL-7.||.||||...
// ....L---J.LJ.LJLJ...

pub struct Solver {
    tiles: Vec<Vec<char>>,
    loop_tiles: HashSet<(i32, i32)>,
}

impl Solver {
    pub fn new(tiles: Vec<Vec<char>>) -> Self {
        Self {
            tiles,
            loop_tiles: HashSet::new(),
        }
    }

    pub fn solve(&mut self) {
        println!("{}", self.build_loop_tiles());
    }

    fn build_loop_tiles(&mut self) -> i32 {
        let point_s = self.find_start_point();
        self.loop_tiles.insert(point_s);

        let mut queue = VecDeque::new();
        queue.push_back(point_s);

        while let Some(point) = queue.pop_front() {
            let point_next = match self.get_tile(point) {
                'S' => self
                    .try_south(point)
                    .or_else(|| self.try_north(point))
                    .or_else(|| self.try_east(point))
                    .or_else(|| self.try_west(point)),
                '|' => self.try_south(point).or_else(|| self.try_north(point)),
                '-' => self.try_east(point).or_else(|| self.try_west(point)),
                'L' => self.try_north(point).or_else(|| self.try_east(point)),
                'J' => self.try_north(point).or_else(|| self.try_west(point)),
                '7' => self.try_south(point).or_else(|| self.try_west(point)),
                'F' => self.try_south(point).or_else(|| self.try_east(point)),
                _ => None,
            };

            point_next.map(|p| {
                self.loop_tiles.insert(p);
                queue.push_back(p);
            });
        }

        self.loop_tiles.len() as i32 / 2
    }

    fn try_south(&self, point: (i32, i32)) -> Option<(i32, i32)> {
        let new_point = (point.0 + 1, point.1);
        if new_point.0 >= self.tiles.len() as i32 || self.loop_tiles.contains(&new_point) {
            return None;
        }

        match self.get_tile(new_point) {
            '|' | 'L' | 'J' => Some(new_point),
            _ => None,
        }
    }

    fn try_north(&self, point: (i32, i32)) -> Option<(i32, i32)> {
        let new_point = (point.0 - 1, point.1);
        if new_point.0 < 0 || self.loop_tiles.contains(&new_point) {
            return None;
        }

        match self.get_tile(new_point) {
            '|' | '7' | 'F' => Some(new_point),
            _ => None,
        }
    }

    fn try_east(&self, point: (i32, i32)) -> Option<(i32, i32)> {
        let new_point = (point.0, point.1 + 1);
        if new_point.1 >= self.tiles[0].len() as i32 || self.loop_tiles.contains(&new_point) {
            return None;
        }

        match self.get_tile(new_point) {
            '-' | '7' | 'J' => Some(new_point),
            _ => None,
        }
    }

    fn try_west(&self, point: (i32, i32)) -> Option<(i32, i32)> {
        let new_point = (point.0, point.1 - 1);
        if new_point.1 < 0 || self.loop_tiles.contains(&new_point) {
            return None;
        }

        match self.get_tile(new_point) {
            '-' | 'L' | 'F' => Some(new_point),
            _ => None,
        }
    }

    fn get_tile(&self, point: (i32, i32)) -> char {
        self.tiles[point.0 as usize][point.1 as usize]
    }

    fn find_start_point(&self) -> (i32, i32) {
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
