impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        let mut m = 0;
        for &v in arr.iter().rev() {
            m = m.max(v);
            ans.push(m);
        }

        ans.pop();
        ans.reverse();
        ans.push(-1);

        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
