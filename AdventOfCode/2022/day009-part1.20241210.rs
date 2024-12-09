use std::collections::{HashMap, HashSet};
use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Option<Direction> {
        match s {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }
}

struct Point {
    pos: (i32, i32),
    tracks: HashSet<(i32, i32)>,
}

impl Point {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            tracks: HashSet::from([(0, 0)]),
        }
    }

    fn move_one_step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.pos.0 -= 1,
            Direction::Down => self.pos.0 += 1,
            Direction::Left => self.pos.1 -= 1,
            Direction::Right => self.pos.1 += 1,
        }
    }

    fn tailgate(&mut self, other: &Self) {
        self.pos = if self.pos.0.abs_diff(other.pos.0) <= 1 && self.pos.1.abs_diff(other.pos.1) <= 1
        {
            self.pos
        } else if self.pos.0 == other.pos.0 || self.pos.1 == other.pos.1 {
            (
                (self.pos.0 + other.pos.0) / 2,
                (self.pos.1 + other.pos.1) / 2,
            )
        } else {
            if self.pos.0.abs_diff(other.pos.0) == 2 {
                ((self.pos.0 + other.pos.0) / 2, other.pos.1)
            } else {
                assert_eq!(self.pos.1.abs_diff(other.pos.1), 2);
                (other.pos.0, (self.pos.1 + other.pos.1) / 2)
            }
        };

        self.tracks.insert(self.pos);
    }
}

fn simulate(movements: &[(Direction, i32)]) -> i32 {
    let mut head = Point::new();
    let mut tail = Point::new();

    for &(direction, steps) in movements {
        for _ in 0..steps {
            head.move_one_step(direction);
            tail.tailgate(&head);
        }
    }

    tail.tracks.len() as i32
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let mut movements: Vec<(Direction, i32)> = vec![];

    while std::io::stdin().read_line(&mut buf)? != 0 {
        let (d, v) = buf.trim().split_once(" ").unwrap();
        movements.push((Direction::from_str(d).unwrap(), v.parse::<i32>().unwrap()));

        buf.clear();
    }

    println!("{}", simulate(&movements));

    Ok(())
}
