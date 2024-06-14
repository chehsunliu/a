impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;

        let mut ans = vec![];
        while l < r {
            let s = numbers[l] + numbers[r];
            if s == target {
                ans.push(l as i32 + 1);
                ans.push(r as i32 + 1);
                break;
            } else if s > target {
                r -= 1;
            } else {
                l += 1;
            }
        }

        ans
    }
}

struct Solution;
