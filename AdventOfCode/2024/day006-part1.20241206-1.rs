use std::collections::HashSet;
use std::io::{self};

pub enum Movement {
    North,
    East,
    South,
    West,
}

impl Movement {
    pub fn step(&self) -> (i32, i32) {
        match self {
            Movement::North => (-1, 0),
            Movement::East => (0, 1),
            Movement::South => (1, 0),
            Movement::West => (0, -1),
        }
    }

    pub fn turn_right(&self) -> Movement {
        match self {
            Movement::North => Movement::East,
            Movement::East => Movement::South,
            Movement::South => Movement::West,
            Movement::West => Movement::North,
        }
    }
}

#[derive(Debug)]
pub struct Solver {
    start: (i32, i32),
    obstructions: HashSet<(i32, i32)>,
    map_size: (i32, i32),
}

impl Solver {
    pub fn new() -> io::Result<Self> {
        let mut buf = String::new();
        let mut obstructions: HashSet<(i32, i32)> = HashSet::new();
        let mut start: Option<(i32, i32)> = None;

        let mut i_max = 0;
        let mut j_max = 0;
        while io::stdin().read_line(&mut buf)? != 0 {
            for (j, c) in buf.trim().chars().enumerate() {
                match c {
                    '#' => {
                        obstructions.insert((i_max, j as i32));
                    }
                    '^' => {
                        start = Some((i_max, j as i32));
                    }
                    _ => {}
                }
            }

            j_max = buf.trim().len() as i32;
            i_max += 1;
            buf.clear();
        }

        Ok(Self {
            start: start.unwrap(),
            obstructions,
            map_size: (i_max, j_max),
        })
    }

    pub fn solve(&self) -> i32 {
        let mut pos = self.start;
        let mut movement = Movement::North;
        let mut tiles: HashSet<(i32, i32)> = HashSet::new();

        loop {
            // println!("{:?} {:?}", pos, self.obstructions);
            tiles.insert(pos);
            let step = movement.step();
            let next_pos = (pos.0 + step.0, pos.1 + step.1);
            if next_pos.0 < 0
                || next_pos.1 < 0
                || next_pos.0 >= self.map_size.0
                || next_pos.1 >= self.map_size.1
            {
                break;
            }

            if self.obstructions.contains(&next_pos) {
                movement = movement.turn_right();
            } else {
                pos = next_pos;
            }
        }

        tiles.len() as i32
    }
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    println!("{}", Solver::new()?.solve());
    Ok(())
}
