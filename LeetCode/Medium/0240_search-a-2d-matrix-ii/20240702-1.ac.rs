impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for i in 0..matrix.len() {
            if search(&matrix[i], target) {
                return true;
            }
        }

        false
    }
}

fn search(nums: &[i32], target: i32) -> bool {
    let (mut l, mut r) = (0, nums.len() as i32 - 1);

    while l <= r {
        let m = l + (r - l) / 2;
        if nums[m as usize] == target {
            return true;
        } else if nums[m as usize] > target {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    false
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
