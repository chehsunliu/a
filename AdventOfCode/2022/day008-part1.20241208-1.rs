struct Vision {
    up: i32,
    down: i32,
    right: i32,
    left: i32,
}

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

    fn is_visible(&self) -> bool {
        self.height > self.vision.right
            || self.height > self.vision.left
            || self.height > self.vision.up
            || self.height > self.vision.down
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
        for j in 1..j_max {
            trees[i][j].vision.left = trees[i][j - 1].vision.left.max(trees[i][j - 1].height);
        }
    }
    for i in 0..i_max {
        for j in (0..j_max - 1).rev() {
            trees[i][j].vision.right = trees[i][j + 1].vision.right.max(trees[i][j + 1].height);
        }
    }
    for j in 0..j_max {
        for i in 1..i_max {
            trees[i][j].vision.up = trees[i - 1][j].vision.up.max(trees[i - 1][j].height);
        }
    }
    for j in 0..j_max {
        for i in (0..i_max - 1).rev() {
            trees[i][j].vision.down = trees[i + 1][j].vision.down.max(trees[i + 1][j].height);
        }
    }

    let mut count = 0;
    for i in 0..i_max {
        for j in 0..j_max {
            count +=
                if i == 0 || i == i_max - 1 || j == 0 || j == j_max - 1 || trees[i][j].is_visible()
                {
                    1
                } else {
                    0
                }
        }
    }

    println!("{}", count);

    Ok(())
}
