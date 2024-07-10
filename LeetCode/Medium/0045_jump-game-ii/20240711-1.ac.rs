impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut pos: usize = 0;
        let mut count = 0;

        while pos < nums.len() - 1 {
            if pos + nums[pos] as usize >= nums.len() - 1 {
                count += 1;
                break;
            }

            let mut v = (pos, 0);
            for i in (pos + 1)..=(pos + nums[pos] as usize) {
                if i + nums[i] as usize > v.1 {
                    v = (i, i + nums[i] as usize);
                }
            }
            pos = v.0;
            count += 1;
        }

        count
    }
}

// 2 7 0 0 6 0 0 1 1 0 1
// ^ ^     ^           ^

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
