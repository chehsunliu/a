impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        
        for target in nums1 {
            let mut prev_value = None;
            for &current_value in nums2.iter().rev() {
                if current_value == target {
                    ans.push(prev_value.unwrap_or(-1));
                    break;
                }
                
                if current_value > target {
                    prev_value = Some(current_value);
                }
            }
        }
        
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
