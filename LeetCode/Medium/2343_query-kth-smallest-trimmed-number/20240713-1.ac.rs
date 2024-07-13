impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];

        for query in queries {
            let (k, trim) = (query[0], query[1]);
            let mut ns: Vec<(&str, i32)> = nums
                .iter()
                .enumerate()
                .map(|(i, s)| (&s[(s.len() - trim as usize)..], i as i32))
                .collect();
            ns.sort();
            ans.push(ns[k as usize - 1].1);
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
