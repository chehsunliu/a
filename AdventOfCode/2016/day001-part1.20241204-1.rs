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

        for action in &actions {
            self.proceed(action)
        }

        self.point.0 + self.point.1
    }

    fn proceed(&mut self, action: &Action) {
        match self.direction {
            Direction::North => match action {
                Action::R(v) => {
                    self.point.0 += v;
                    self.direction = Direction::East;
                }
                Action::L(v) => {
                    self.point.0 -= v;
                    self.direction = Direction::West;
                }
            },
            Direction::South => match action {
                Action::R(v) => {
                    self.point.0 -= v;
                    self.direction = Direction::West;
                }
                Action::L(v) => {
                    self.point.0 += v;
                    self.direction = Direction::East;
                }
            },
            Direction::West => match action {
                Action::R(v) => {
                    self.point.1 += v;
                    self.direction = Direction::North;
                }
                Action::L(v) => {
                    self.point.1 -= v;
                    self.direction = Direction::South;
                }
            },
            Direction::East => match action {
                Action::R(v) => {
                    self.point.1 -= v;
                    self.direction = Direction::South;
                }
                Action::L(v) => {
                    self.point.1 += v;
                    self.direction = Direction::North;
                }
            },
        }
    }
}
