impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let lookup = s.chars().collect::<Vec<char>>();
        let mut s = lookup.clone();
        
        for (i, _) in lookup.iter().enumerate() {
            s[i] = lookup[(i + k as usize) % lookup.len()];
        }
        
        s.into_iter().collect()
    }
}