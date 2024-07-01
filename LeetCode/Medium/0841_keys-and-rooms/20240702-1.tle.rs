use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut keys: HashSet<i32> = HashSet::new();
        let mut visited: HashSet<i32> = HashSet::new();
        let mut queue: VecDeque<i32> = VecDeque::new();

        queue.push_back(0);

        while let Some(i) = queue.pop_front() {
            visited.insert(i);
            for key in &rooms[i as usize] {
                keys.insert(*key);
            }

            for key in &keys {
                if !visited.contains(key) {
                    queue.push_back(*key);
                }
            }
        }

        visited.len() == rooms.len()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
