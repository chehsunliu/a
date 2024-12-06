use std::collections::{HashMap, HashSet};
use std::io::{self};

#[derive(Debug)]
pub struct Solver {
    rules: HashMap<i32, HashSet<i32>>,
}

impl Solver {
    pub fn new(rules: HashMap<i32, HashSet<i32>>) -> Solver {
        Self { rules }
    }

    pub fn solve(&self, values: Vec<i32>) -> i32 {
        for i in 0..values.len() {
            for j in i + 1..values.len() {
                let (v1, v2) = (values[i], values[j]);

                if let Some(set) = self.rules.get(&v2) {
                    if set.contains(&v1) {
                        return 0;
                    }
                }
            }
        }

        values[values.len() / 2]
    }
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let rules = create_rules()?;

    let solver = Solver::new(rules);
    let mut ans = 0;
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        let values = buf
            .trim()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        ans += solver.solve(values);
        buf.clear();
    }

    println!("{}", ans);

    Ok(())
}

fn create_rules() -> io::Result<HashMap<i32, HashSet<i32>>> {
    let mut buf = String::new();
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    while io::stdin().read_line(&mut buf)? != 0 {
        if buf.trim().is_empty() {
            buf.clear();
            break;
        }

        let (t1, t2) = buf.trim().split_once('|').unwrap();
        let (t1, t2) = (t1.parse::<i32>().unwrap(), t2.parse::<i32>().unwrap());

        if let Some(values) = rules.get_mut(&t1) {
            values.insert(t2);
        } else {
            rules.insert(t1, HashSet::from_iter([t2]));
        }

        buf.clear();
    }

    Ok(rules)
}
