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
    // 10313550
    println!("{}", solver.solve());

    Ok(())
}

#[derive(PartialEq)]
enum Movement {
    Horizontal,
    Vertical,
}

struct Solver {
    img: Vec<Vec<char>>,
    g_rows: HashSet<i32>,
    g_cols: HashSet<i32>,
}

impl Solver {
    fn new(img: Vec<Vec<char>>) -> Self {
        let mut g_rows = HashSet::new();
        for i in 0..img.len() {
            if !img[i].iter().any(|c| *c == '#') {
                g_rows.insert(i as i32);
            }
        }

        let mut g_cols = HashSet::new();
        for j in 0..img[0].len() {
            let col = (0..img.len()).map(|i| img[i][j]).collect::<Vec<char>>();
            if !col.iter().any(|c| *c == '#') {
                g_cols.insert(j as i32);
            }
        }

        Self {
            img,
            g_rows,
            g_cols,
        }
    }

    fn solve(&self) -> i64 {
        let mut ans = 0;

        let galaxies = self.find_galaxies();
        for &galaxy in &galaxies {
            let mut distances = vec![vec![-1; self.img[0].len()]; self.img.len()];

            let mut queue: VecDeque<(Option<(Movement, i64)>, (i32, i32))> = VecDeque::new();
            queue.push_back((None, galaxy));

            while let Some((info, p)) = queue.pop_front() {
                if !self.is_valid_indexes(p) || distances[p.0 as usize][p.1 as usize] != -1 {
                    continue;
                }

                let d = if let Some((movement, d)) = info {
                    if self.g_rows.contains(&p.0) && movement == Movement::Vertical {
                        d + 1000000
                    } else if self.g_cols.contains(&p.1) && movement == Movement::Horizontal {
                        d + 1000000
                    } else {
                        d + 1
                    }
                } else {
                    0
                };

                distances[p.0 as usize][p.1 as usize] = d;
                if self.img[p.0 as usize][p.1 as usize] == '#' {
                    ans += d;
                }

                queue.push_back((Some((Movement::Vertical, d)), (p.0 - 1, p.1)));
                queue.push_back((Some((Movement::Vertical, d)), (p.0 + 1, p.1)));
                queue.push_back((Some((Movement::Horizontal, d)), (p.0, p.1 - 1)));
                queue.push_back((Some((Movement::Horizontal, d)), (p.0, p.1 + 1)));
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
