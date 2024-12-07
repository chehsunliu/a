use std::cmp::Ordering;
use std::collections::HashSet;

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> std::io::Result<()> {
    let mut buf = String::new();

    let mut rules: HashSet<String> = HashSet::new();
    while std::io::stdin().read_line(&mut buf)? != 0 {
        if buf.trim().is_empty() {
            buf.clear();
            break;
        }

        rules.insert(buf.trim().to_string());
        buf.clear();
    }

    let mut updates: Vec<Vec<i32>> = vec![];
    while std::io::stdin().read_line(&mut buf)? != 0 {
        updates.push(
            buf.trim()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
        buf.clear();
    }

    println!("{}", solve(&rules, updates));

    Ok(())
}

fn solve(rules: &HashSet<String>, updates: Vec<Vec<i32>>) -> i32 {
    let updates = updates
        .into_iter()
        .filter(|u| is_incorrect(rules, u))
        .collect::<Vec<_>>();

    let mut ans = 0;
    for mut update in updates {
        update.sort_by(|a, b| {
            if rules.contains(&format!("{}|{}", *b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        ans += update[update.len() / 2];
    }

    ans
}

fn is_incorrect(rules: &HashSet<String>, update: &Vec<i32>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let (v1, v2) = (update[i], update[j]);
            if rules.contains(&format!("{}|{}", v2, v1)) {
                return true;
            }
        }
    }

    false
}
