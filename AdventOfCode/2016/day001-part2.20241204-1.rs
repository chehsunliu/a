use std::collections::HashSet;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        println!("{}", Solver::new().solve(buf.trim().to_string()));
        buf.clear();
    }

    Ok(())
}

enum Action {
    R(i32),
    L(i32),
}

#[derive(Debug)]
enum Direction {
    North,
    West,
    East,
    South,
}

#[derive(Debug)]
pub struct Solver {
    direction: Direction,
    point: (i32, i32),
}

impl Solver {
    pub fn new() -> Solver {
        Self {
            direction: Direction::North,
            point: (0, 0),
        }
    }

    pub fn solve(&mut self, s: String) -> i32 {
        let actions: Vec<Action> = s
            .split(',')
            .map(|s| s.trim())
            .map(|s| {
                let c = s.chars().nth(0).unwrap();
                let v: i32 = s[1..].parse().unwrap();
                if c == 'R' {
                    Action::R(v)
                } else {
                    Action::L(v)
                }
            })
            .collect();

        let mut note: HashSet<(i32, i32)> = HashSet::new();
        note.insert((0, 0));

        for action in &actions {
            if let Some(p) = self.proceed(action, &mut note) {
                return p.0 + p.1;
            }
        }

        panic!("GG")
    }

    fn proceed(&mut self, action: &Action, note: &mut HashSet<(i32, i32)>) -> Option<(i32, i32)> {
        let offset = match self.direction {
            Direction::North => match action {
                Action::R(v) => {
                    self.direction = Direction::East;
                    (1, 0, v)
                }
                Action::L(v) => {
                    self.direction = Direction::West;
                    (-1, 0, v)
                }
            },
            Direction::South => match action {
                Action::R(v) => {
                    self.direction = Direction::West;
                    (-1, 0, v)
                }
                Action::L(v) => {
                    self.direction = Direction::East;
                    (1, 0, v)
                }
            },
            Direction::West => match action {
                Action::R(v) => {
                    self.direction = Direction::North;
                    (0, 1, v)
                }
                Action::L(v) => {
                    self.direction = Direction::South;
                    (0, -1, v)
                }
            },
            Direction::East => match action {
                Action::R(v) => {
                    self.direction = Direction::South;
                    (0, -1, v)
                }
                Action::L(v) => {
                    self.direction = Direction::North;
                    (0, 1, v)
                }
            },
        };

        let (i, j, &v) = offset;

        for _ in 0..v {
            self.point = (self.point.0 + i, self.point.1 + j);

            if note.contains(&self.point) {
                return Some(self.point);
            }

            note.insert(self.point);
        }

        None
    }
}
