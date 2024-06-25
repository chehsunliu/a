impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            n => Solution::fib(n - 1) + Solution::fib(n - 2),
        }
    }
}

struct Solution;
