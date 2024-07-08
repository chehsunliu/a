use std::cmp::max;
use std::io::{self};

struct Node {
    val: (i32, i32),
    max: i32,
    next: Option<(Box<Node>, Box<Node>)>,
}

impl Node {
    fn new(val: (i32, i32)) -> Self {
        Self {
            val,
            max: val.1 - val.0 + 1,
            next: None,
        }
    }

    fn split(&mut self, p: i32) -> i32 {
        if let Some((l, r)) = self.next.as_mut() {
            if p <= l.val.1 {
                self.max = max(r.max, l.split(p));
            } else {
                self.max = max(l.max, r.split(p));
            };
            return self.max;
        }

        let l = Box::new(Node::new((self.val.0, p - 1)));
        let r = Box::new(Node::new((p, self.val.1)));
        self.max = max(p - self.val.0, self.val.1 - p + 1);
        self.next = Some((l, r));

        self.max
    }
}

fn solve(x: i32, ps: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];

    let mut root = Node::new((0, x - 1));
    for p in ps {
        ans.push(root.split(p));
    }

    ans
}

// 0 1 2 3 4 5 6 7 8
//  _ _ _ _ _ _ _ _
//       ^
//       ^     ^
//     ^ ^     ^

// 0 1 2 . 3 4 5 6 7
// 0 1 2 . 3 4 5 . 6 7
// 0 1 . 2 . 3 4 5 . 6 7

//            (0, 7)
//      (0, 2)     (3, 7)

//            (0, 7)
//      (0, 2)     (3, 7)
//              (3, 5) (6, 7)

//              (0, 7)
//        (0, 2)       (3, 7)
//    (0, 1) (2, 2)   (3, 5) (6, 7)

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;
    let (x, n) = buf.trim().split_once(' ').unwrap();
    let (x, n) = (x.parse::<i32>().unwrap(), n.parse::<usize>().unwrap());
    buf.clear();

    io::stdin().read_line(&mut buf)?;
    let ps = buf
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    buf.clear();
    assert_eq!(ps.len(), n);

    let result = solve(x, ps)
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);

    Ok(())
}
