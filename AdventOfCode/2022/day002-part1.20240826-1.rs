use std::io::{self};

pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn from_chars(s: &str) -> Self {
        match s {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> i32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    pub fn compete(&self, other: &Self) -> i32 {
        let score = match self {
            Play::Rock => match other {
                Play::Rock => 3,
                Play::Paper => 0,
                Play::Scissors => 6,
            },
            Play::Paper => match other {
                Play::Rock => 6,
                Play::Paper => 3,
                Play::Scissors => 0,
            },
            Play::Scissors => match other {
                Play::Rock => 0,
                Play::Paper => 6,
                Play::Scissors => 3,
            },
        };

        score + self.score()
    }
}

fn solve(plays: Vec<(Play, Play)>) -> i32 {
    let mut score = 0;

    for (p1, p2) in plays.iter() {
        score += p2.compete(p1);
    }

    score
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut plays: Vec<(Play, Play)> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        let (s1, s2) = buf.trim().split_once(" ").unwrap();
        plays.push((Play::from_chars(s1), Play::from_chars(s2)));
        buf.clear();
    }

    println!("{}", solve(plays));

    Ok(())
}
