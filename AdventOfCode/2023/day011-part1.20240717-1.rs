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

    let solver = Solver::new(img);
    println!("{}", solver.solve());

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

    fn solve(&self) -> i32 {
        let mut ans = 0;

        let galaxies = self.find_galaxies();
        for &galaxy in &galaxies {
            let mut distances = vec![vec![i32::MAX; self.img[0].len()]; self.img.len()];

            let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();
            queue.push_back((galaxy, 0));

            while let Some((p, d)) = queue.pop_front() {
                if !self.is_valid_indexes(p) || distances[p.0 as usize][p.1 as usize] != i32::MAX {
                    continue;
                }

                distances[p.0 as usize][p.1 as usize] = d;
                if self.img[p.0 as usize][p.1 as usize] == '#' {
                    ans += d;
                }

                queue.push_back(((p.0 - 1, p.1), d + 1));
                queue.push_back(((p.0 + 1, p.1), d + 1));
                queue.push_back(((p.0, p.1 - 1), d + 1));
                queue.push_back(((p.0, p.1 + 1), d + 1));
            }
        }

        ans / 2
    }

    fn is_valid_indexes(&self, p: (i32, i32)) -> bool {
        p.0 >= 0 && p.0 < self.img.len() as i32 && p.1 >= 0 && p.1 < self.img[0].len() as i32
    }

    fn find_galaxies(&self) -> Vec<(i32, i32)> {
        let mut galaxies = vec![];

        for i in 0..self.img.len() {
            for j in 0..self.img[0].len() {
                if self.img[i][j] == '#' {
                    galaxies.push((i as i32, j as i32));
                }
            }
        }

        galaxies
    }

    fn print(&self) {
        for row in &self.img {
            println!("{}", row.into_iter().collect::<String>());
        }
    }
}
