impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = vec![0; 100001];
        for v in &nums {
            counts[(*v + 50000) as usize] += 1;
        }

        let mut ans = Vec::with_capacity(nums.len());
        for (v, count) in counts.iter().enumerate() {
            for _ in 0..*count {
                ans.push(v as i32 - 50000);
            }
        }

        ans
    }
}

// 1 2 4 5 4 9 3

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::sort_array(vec![4, 1, 2, 3]), vec![1, 2, 3, 4]);
    }
}
