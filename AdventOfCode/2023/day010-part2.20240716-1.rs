use std::collections::{HashMap, HashSet, VecDeque};
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

// 0 | 1

// 0
// -
// 1

// 0 L 1

// 0 J 1

// 0 7 1

// 0 F 1

//   0
// 0 S 1
//   1

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
    loop_tiles: HashMap<(i32, i32), i32>,
    border_tiles: HashMap<(i32, i32), i32>,
}

impl Solver {
    pub fn new(tiles: Vec<Vec<char>>) -> Self {
        Self {
            tiles,
            loop_tiles: HashMap::new(),
            border_tiles: HashMap::new(),
        }
    }

    pub fn solve(&mut self) {
        let loop_tiles_count = self.build_loop_tiles();
        self.build_complete_border_tiles();

        let mut inner_tiles_count = 0;
        for (_, v) in &self.border_tiles {
            inner_tiles_count += v;
        }

        self.print();
        println!("{} {}", loop_tiles_count, inner_tiles_count);
    }

    fn print(&self) {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                let p = (i as i32, j as i32);
                if self.loop_tiles.contains_key(&p) {
                    print!(".");
                    // print!("{}", self.tiles[p.0 as usize][p.1 as usize]);
                } else if self.border_tiles.contains_key(&p) {
                    print!("{}", self.border_tiles.get(&p).unwrap());
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn build_complete_border_tiles(&mut self) {
        let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();

        for (&p, &v) in &self.border_tiles {
            queue.push_back(((p.0 - 1, p.1), v));
            queue.push_back(((p.0 + 1, p.1), v));
            queue.push_back(((p.0, p.1 - 1), v));
            queue.push_back(((p.0, p.1 + 1), v));
        }

        while let Some((p, v)) = queue.pop_front() {
            if !self.is_valid_range(p)
                || self.loop_tiles.contains_key(&p)
                || self.border_tiles.contains_key(&p)
            {
                continue;
            }

            self.border_tiles.insert(p, v);
            queue.push_back(((p.0 - 1, p.1), v));
            queue.push_back(((p.0 + 1, p.1), v));
            queue.push_back(((p.0, p.1 - 1), v));
            queue.push_back(((p.0, p.1 + 1), v));
        }
    }

    fn build_loop_tiles(&mut self) -> i32 {
        let p_start = self.find_start_point();
        self.loop_tiles.insert(p_start, 0);

        let mut point = Some(p_start);

        while let Some(p) = point {
            self.draw_border(p);

            point = self
                .try_south(p)
                .or_else(|| self.try_north(p))
                .or_else(|| self.try_east(p))
                .or_else(|| self.try_west(p));
        }

        self.loop_tiles.len() as i32 / 2
    }

    fn draw_border(&mut self, p: (i32, i32)) {
        self.border_tiles.remove(&p);

        let tile = self.get_tile(p);
        let tile_v = *self.loop_tiles.get(&p).unwrap();
        match tile {
            'S' => {
                let pb = (p.0 - 1, p.1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v);
                }

                let pb = (p.0 + 1, p.1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v ^ 1);
                }

                let pb = (p.0, p.1 - 1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v);
                }

                let pb = (p.0, p.1 + 1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v ^ 1);
                }
            }
            '|' => {
                let pb = (p.0, p.1 - 1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v);
                }

                let pb = (p.0, p.1 + 1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v ^ 1);
                }
            }
            '-' => {
                let pb = (p.0 - 1, p.1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v);
                }

                let pb = (p.0 + 1, p.1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v ^ 1);
                }
            }
            'L' => {
                let pb = (p.0, p.1 - 1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v);
                }

                let pb = (p.0 + 1, p.1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v);
                }
            }
            'J' => {
                let pb = (p.0, p.1 + 1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v ^ 1);
                }

                let pb = (p.0 + 1, p.1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v ^ 1);
                }
            }
            '7' => {
                let pb = (p.0 - 1, p.1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v ^ 1);
                }

                let pb = (p.0, p.1 + 1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v ^ 1);
                }
            }
            'F' => {
                let pb = (p.0 - 1, p.1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v);
                }

                let pb = (p.0, p.1 - 1);
                if self.is_valid_range(pb) && !self.loop_tiles.contains_key(&pb) {
                    self.border_tiles.insert(pb, tile_v);
                }
            }
            _ => unreachable!(),
        }
    }

    fn try_south(&mut self, point: (i32, i32)) -> Option<(i32, i32)> {
        let new_point = (point.0 + 1, point.1);
        if !self.is_valid_range(new_point) || self.loop_tiles.contains_key(&new_point) {
            return None;
        }

        let tile = self.get_tile(point);
        let new_tile = self.get_tile(new_point);

        if tile != 'S' && tile != '|' && tile != '7' && tile != 'F' {
            return None;
        }

        let tile_v = *self.loop_tiles.get(&point).unwrap();

        match new_tile {
            '|' | 'L' | 'J' => {
                self.loop_tiles.insert(new_point, tile_v);
                Some(new_point)
            }
            _ => None,
        }
    }

    fn try_north(&mut self, point: (i32, i32)) -> Option<(i32, i32)> {
        let new_point = (point.0 - 1, point.1);
        if !self.is_valid_range(new_point) || self.loop_tiles.contains_key(&new_point) {
            return None;
        }

        let tile = self.get_tile(point);
        let new_tile = self.get_tile(new_point);

        if tile != 'S' && tile != '|' && tile != 'L' && tile != 'J' {
            return None;
        }

        let tile_v = *self.loop_tiles.get(&point).unwrap();

        match new_tile {
            '|' | '7' | 'F' => {
                self.loop_tiles.insert(new_point, tile_v);
                Some(new_point)
            }
            _ => None,
        }
    }

    fn try_east(&mut self, point: (i32, i32)) -> Option<(i32, i32)> {
        let new_point = (point.0, point.1 + 1);
        if !self.is_valid_range(new_point) || self.loop_tiles.contains_key(&new_point) {
            return None;
        }

        let tile = self.get_tile(point);
        let new_tile = self.get_tile(new_point);

        if tile != 'S' && tile != '-' && tile != 'L' && tile != 'F' {
            return None;
        }

        let tile_v = *self.loop_tiles.get(&point).unwrap();

        match new_tile {
            '-' => {
                match tile {
                    'S' | '-' | 'F' => self.loop_tiles.insert(new_point, tile_v),
                    'L' => self.loop_tiles.insert(new_point, tile_v ^ 1),
                    _ => unreachable!(),
                };
                Some(new_point)
            }
            '7' => {
                match tile {
                    'L' => self.loop_tiles.insert(new_point, tile_v),
                    'S' | '-' | 'F' => self.loop_tiles.insert(new_point, tile_v ^ 1),
                    _ => unreachable!(),
                };
                Some(new_point)
            }
            'J' => {
                match tile {
                    'S' | '-' | 'F' => self.loop_tiles.insert(new_point, tile_v),
                    'L' => self.loop_tiles.insert(new_point, tile_v ^ 1),
                    _ => unreachable!(),
                };
                Some(new_point)
            }
            _ => None,
        }
    }

    fn try_west(&mut self, point: (i32, i32)) -> Option<(i32, i32)> {
        let new_point = (point.0, point.1 - 1);
        if !self.is_valid_range(new_point) || self.loop_tiles.contains_key(&new_point) {
            return None;
        }

        let tile = self.get_tile(point);
        let new_tile = self.get_tile(new_point);

        if tile != 'S' && tile != '-' && tile != '7' && tile != 'J' {
            return None;
        }

        let tile_v = *self.loop_tiles.get(&point).unwrap();

        match new_tile {
            '-' => {
                match tile {
                    'S' | '-' | 'J' => self.loop_tiles.insert(new_point, tile_v),
                    '7' => self.loop_tiles.insert(new_point, tile_v ^ 1),
                    _ => unreachable!(),
                };
                Some(new_point)
            }
            'L' => {
                match tile {
                    '7' => self.loop_tiles.insert(new_point, tile_v),
                    'S' | '-' | 'J' => self.loop_tiles.insert(new_point, tile_v ^ 1),
                    _ => unreachable!(),
                };
                Some(new_point)
            }
            'F' => {
                match tile {
                    'S' | '-' | 'J' => self.loop_tiles.insert(new_point, tile_v),
                    '7' => self.loop_tiles.insert(new_point, tile_v ^ 1),
                    _ => unreachable!(),
                };
                Some(new_point)
            }
            _ => None,
        }
    }

    fn get_tile(&self, point: (i32, i32)) -> char {
        self.tiles[point.0 as usize][point.1 as usize]
    }

    fn is_valid_range(&self, p: (i32, i32)) -> bool {
        p.0 >= 0 && p.0 < self.tiles.len() as i32 && p.1 >= 0 && p.1 < self.tiles[0].len() as i32
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
