use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut lookup: HashMap<[u8; 26], Vec<&str>> = HashMap::new();
        for s in &strs {
            let mut counter: [u8; 26] = [0; 26];
            s.chars()
                .for_each(|c| counter[(c as u8 - b'a') as usize] += 1);

            if let Some(vs) = lookup.get_mut(&counter) {
                vs.push(s);
            } else {
                lookup.insert(counter, vec![s]);
            }
        }

        let mut ans: Vec<Vec<String>> = vec![];

        lookup.values().for_each(|vs| {
            ans.push(vs.iter().map(|v| v.to_string()).collect::<Vec<String>>());
        });

        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
