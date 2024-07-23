impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = [0; 202];

        for num in &nums {
            counts[(*num + 100) as usize] += 1;
        }

        let mut nums = vec![];
        for i in 0..counts.len() {
            if counts[i] != 0 {
                nums.push((counts[i], i as i32 - 100));
            }
        }

        nums.sort_by_key(|num| (num.0, -num.1));

        let mut ans = vec![];
        for &(count, v) in &nums {
            for _ in 0..count {
                ans.push(v);
            }
        }

        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
