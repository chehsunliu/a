use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut img: Vec<Vec<char>> = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        img.push(buf.trim().chars().collect());
        buf.clear();
    }

    Solver::new(img).print();

    Ok(())
}

struct Solver {
    img: Vec<Vec<char>>,
}

impl Solver {
    fn new(mut img: Vec<Vec<char>>) -> Self {
        for _ in 0..2 {
            let mut img_t = vec![];
            for j in 0..img[0].len() {
                let col = (0..img.len()).map(|i| img[i][j]).collect::<Vec<char>>();
                if !col.iter().any(|c| *c == '#') {
                    img_t.push(col.clone());
                }
                img_t.push(col);
            }
            img = img_t;
        }

        Self { img }
    }

    fn print(&self) {
        for row in &self.img {
            println!("{}", row.into_iter().collect::<String>());
        }
    }
}
