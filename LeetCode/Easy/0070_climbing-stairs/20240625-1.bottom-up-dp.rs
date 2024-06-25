impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut f1, mut f0) = (1, 1);
        
        for _ in 1..n {
            (f1, f0) = (f1 + f0, f1);
        }
        
        f1
    }
}

struct Solution;
