use std::collections::VecDeque;

#[derive(Debug)]
struct Vision {
    up: i32,
    down: i32,
    right: i32,
    left: i32,
}

#[derive(Debug)]
struct Tree {
    height: i32,
    vision: Vision,
}

impl Tree {
    fn new(height: i32) -> Tree {
        Self {
            height,
            vision: Vision {
                up: 0,
                down: 0,
                right: 0,
                left: 0,
            },
        }
    }
}

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let mut trees: Vec<Vec<Tree>> = Vec::new();

    while std::io::stdin().read_line(&mut buf)? != 0 {
        trees.push(
            buf.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .map(|h| Tree::new(h))
                .collect(),
        );
        buf.clear();
    }

    let (i_max, j_max) = (trees.len(), trees[0].len());

    for i in 0..i_max {
        let mut stack: VecDeque<usize> = VecDeque::new();
        stack.push_back(0);

        for j in 1..j_max {
            while stack.back().map_or(false, |&j_back| {
                trees[i][j_back].height <= trees[i][j].height
            }) {
                let j_popped = stack.pop_back().unwrap();
                trees[i][j_popped].vision.right = (j - j_popped) as i32;
            }
            stack.push_back(j);
        }

        while let Some(j_popped) = stack.pop_back() {
            trees[i][j_popped].vision.right = (j_max - 1 - j_popped) as i32;
        }
    }

    for i in 0..i_max {
        let mut stack: VecDeque<usize> = VecDeque::new();
        stack.push_back(j_max - 1);

        for j in (0..j_max - 1).rev() {
            while stack.back().map_or(false, |&j_back| {
                trees[i][j_back].height <= trees[i][j].height
            }) {
                let j_popped = stack.pop_back().unwrap();
                trees[i][j_popped].vision.left = (j_popped - j) as i32;
            }
            stack.push_back(j);
        }

        while let Some(j_popped) = stack.pop_back() {
            trees[i][j_popped].vision.left = (j_popped - 0) as i32;
        }
    }

    for j in 0..j_max {
        let mut stack: VecDeque<usize> = VecDeque::new();
        stack.push_back(0);

        for i in 1..i_max {
            while stack.back().map_or(false, |&i_back| {
                trees[i_back][j].height <= trees[i][j].height
            }) {
                let i_popped = stack.pop_back().unwrap();
                trees[i_popped][j].vision.down = (i - i_popped) as i32;
            }
            stack.push_back(i);
        }

        while let Some(i_popped) = stack.pop_back() {
            trees[i_popped][j].vision.down = (i_max - 1 - i_popped) as i32;
        }
    }

    for j in 0..j_max {
        let mut stack: VecDeque<usize> = VecDeque::new();
        stack.push_back(i_max - 1);

        for i in (0..i_max - 1).rev() {
            while stack.back().map_or(false, |&i_back| {
                trees[i_back][j].height <= trees[i][j].height
            }) {
                let i_popped = stack.pop_back().unwrap();
                trees[i_popped][j].vision.up = (i_popped - i) as i32;
            }
            stack.push_back(i);
        }

        while let Some(i_popped) = stack.pop_back() {
            trees[i_popped][j].vision.up = (i_popped - 0) as i32;
        }
    }

    let mut ans = 0;
    for i in 0..i_max {
        for j in 0..j_max {
            let tree = &trees[i][j];
            ans = ans.max(tree.vision.up * tree.vision.left * tree.vision.down * tree.vision.right);
        }
    }

    println!("{}", ans);

    Ok(())
}

/*
5 3 5
5 3 2 1 2 1 3

5 (3)
5 3 (2)
5 3 2 (1)
5 3 2 1 (2)
5 3 2 (2) -> 1
5 3 2 -> 2
 */
