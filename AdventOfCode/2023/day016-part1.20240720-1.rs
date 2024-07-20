use std::collections::{HashSet, VecDeque};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut grid: Vec<Vec<char>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        grid.push(buf.trim().chars().collect());
        buf.clear();
    }

    println!("{}", Solver::new(grid).solve());

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    Upward,
    Downward,
    Left,
    Right,
}

pub struct Solver {
    grid: Vec<Vec<char>>,
}

impl Solver {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    pub fn solve(&self) -> i32 {
        let mut visited: HashSet<((i32, i32), Direction)> = HashSet::new();

        let mut queue: VecDeque<((i32, i32), Direction)> = VecDeque::new();
        queue.push_back(((0, 0), Direction::Right));

        while let Some((pos, direction)) = queue.pop_front() {
            if pos.0 < 0
                || pos.0 >= self.grid.len() as i32
                || pos.1 < 0
                || pos.1 >= self.grid[0].len() as i32
                || visited.contains(&(pos, direction))
            {
                continue;
            }

            visited.insert((pos, direction));

            let next_poses = match self.grid[pos.0 as usize][pos.1 as usize] {
                '|' => match direction {
                    Direction::Upward => vec![try_upward(pos)],
                    Direction::Downward => vec![try_downward(pos)],
                    Direction::Left | Direction::Right => vec![try_upward(pos), try_downward(pos)],
                },
                '-' => match direction {
                    Direction::Upward | Direction::Downward => vec![try_left(pos), try_right(pos)],
                    Direction::Left => vec![try_left(pos)],
                    Direction::Right => vec![try_right(pos)],
                },
                '\\' => match direction {
                    Direction::Upward => vec![try_left(pos)],
                    Direction::Downward => vec![try_right(pos)],
                    Direction::Left => vec![try_upward(pos)],
                    Direction::Right => vec![try_downward(pos)],
                },
                '/' => match direction {
                    Direction::Upward => vec![try_right(pos)],
                    Direction::Downward => vec![try_left(pos)],
                    Direction::Left => vec![try_downward(pos)],
                    Direction::Right => vec![try_upward(pos)],
                },
                _ => match direction {
                    Direction::Upward => vec![try_upward(pos)],
                    Direction::Downward => vec![try_downward(pos)],
                    Direction::Left => vec![try_left(pos)],
                    Direction::Right => vec![try_right(pos)],
                },
            };
            next_poses.iter().for_each(|item| queue.push_back(*item));
        }

        visited
            .iter()
            .map(|(pos, _)| *pos)
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

fn try_upward(pos: (i32, i32)) -> ((i32, i32), Direction) {
    ((pos.0 - 1, pos.1), Direction::Upward)
}

fn try_downward(pos: (i32, i32)) -> ((i32, i32), Direction) {
    ((pos.0 + 1, pos.1), Direction::Downward)
}

fn try_left(pos: (i32, i32)) -> ((i32, i32), Direction) {
    ((pos.0, pos.1 - 1), Direction::Left)
}

fn try_right(pos: (i32, i32)) -> ((i32, i32), Direction) {
    ((pos.0, pos.1 + 1), Direction::Right)
}
