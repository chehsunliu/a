impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if i.abs_diff(j) <= (index_diff as usize)
                    && nums[i].abs_diff(nums[j]) <= (value_diff as u32)
                {
                    return true;
                }
            }
        }

        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
