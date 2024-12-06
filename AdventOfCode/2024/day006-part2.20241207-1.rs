use std::collections::{HashMap, HashSet};
use std::io::{self};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
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

struct FirstInfo {
    index: usize,
    movement: Movement,
    prev_pos: (i32, i32),
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
        let (first_infos, complete_infos) = self.find_original_path();
        let mut count = 0;
        let mut used_positions: HashSet<(i32, i32)> = HashSet::new();

        for (pos, first_info) in &first_infos {
            if used_positions.contains(pos) {
                continue;
            }

            used_positions.insert(*pos);

            if self.check(first_info, &complete_infos, *pos) {
                count += 1;
            }
        }

        count
    }

    fn find_original_path(
        &self,
    ) -> (
        HashMap<(i32, i32), FirstInfo>,
        HashMap<(Movement, (i32, i32)), usize>,
    ) {
        let mut pos = self.start;
        let mut movement = Movement::North;
        let mut first_infos: HashMap<(i32, i32), FirstInfo> = HashMap::new();
        let mut complete_infos: HashMap<(Movement, (i32, i32)), usize> = HashMap::new();

        loop {
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
                if !first_infos.contains_key(&next_pos) {
                    first_infos.insert(
                        next_pos,
                        FirstInfo {
                            index: complete_infos.len(),
                            movement,
                            prev_pos: pos,
                        },
                    );
                }

                complete_infos.insert((movement, next_pos), complete_infos.len());
                pos = next_pos;
            }
        }

        (first_infos, complete_infos)
    }

    fn check(
        &self,
        first_info: &FirstInfo,
        complete_infos: &HashMap<(Movement, (i32, i32)), usize>,
        new_obstruction: (i32, i32),
    ) -> bool {
        let mut movement = first_info.movement;
        let mut pos = first_info.prev_pos;
        let mut path: HashSet<(Movement, (i32, i32))> = HashSet::new();

        path.insert((movement, pos));

        loop {
            let step = movement.step();
            let next_pos = (pos.0 + step.0, pos.1 + step.1);
            if next_pos.0 < 0
                || next_pos.1 < 0
                || next_pos.0 >= self.map_size.0
                || next_pos.1 >= self.map_size.1
            {
                break;
            }

            if path.contains(&(movement, next_pos)) {
                return true;
            }

            if self.obstructions.contains(&next_pos) || next_pos == new_obstruction {
                movement = movement.turn_right();
                continue;
            }

            if let Some(&index) = complete_infos.get(&(movement, next_pos)) {
                if index < first_info.index {
                    return true;
                }
            }

            pos = next_pos;
            path.insert((movement, next_pos));
        }

        false
    }
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    println!("{}", Solver::new()?.solve());
    Ok(())
}
