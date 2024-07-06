impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let n = n - 1;
        let time = time - 1;

        let r = time / n;
        if r % 2 == 0 {
            time % n + 2
        } else {
            n - time % n
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
