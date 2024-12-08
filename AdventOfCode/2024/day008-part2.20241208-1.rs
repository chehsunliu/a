use std::collections::{HashMap, HashSet};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let mut i_max = 0;
    let mut j_max = 0;
    while std::io::stdin().read_line(&mut buf)? != 0 {
        for (j, c) in buf.trim().chars().enumerate() {
            if c == '.' {
                continue;
            }

            if let Some(points) = antennas.get_mut(&c) {
                points.push((i_max, j as i32));
            } else {
                antennas.insert(c, vec![(i_max, j as i32)]);
            }
        }

        i_max += 1;
        j_max = j_max.max(buf.trim().len() as i32);
        buf.clear();
    }

    let mut valid_points: HashSet<(i32, i32)> = HashSet::new();

    for points in antennas.values() {
        calculate((i_max, j_max), points, &mut valid_points);
    }

    println!("{}", valid_points.len());

    Ok(())
}

fn calculate(
    boundary: (i32, i32),
    points: &Vec<(i32, i32)>,
    valid_points: &mut HashSet<(i32, i32)>,
) {
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (p1, p2) = (points[i], points[j]);
            let diff = (p2.0 - p1.0, p2.1 - p1.1);

            let mut p = p2;
            while p.0 >= 0 && p.0 < boundary.0 && p.1 >= 0 && p.1 < boundary.1 {
                valid_points.insert(p);
                p = (p.0 + diff.0, p.1 + diff.1);
            }

            let mut p = p2;
            while p.0 >= 0 && p.0 < boundary.0 && p.1 >= 0 && p.1 < boundary.1 {
                valid_points.insert(p);
                p = (p.0 - diff.0, p.1 - diff.1);
            }
        }
    }
}
