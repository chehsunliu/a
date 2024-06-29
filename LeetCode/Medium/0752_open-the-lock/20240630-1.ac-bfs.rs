use std::collections::{HashMap, HashSet, VecDeque};

type Slot = (u8, u8, u8, u8);

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadends = deadends
            .iter()
            .map(|s| to_slot(&s))
            .collect::<HashSet<Slot>>();
        let target = to_slot(&target);
        solve(deadends, target)
    }
}

fn solve(deadends: HashSet<Slot>, target: Slot) -> i32 {
    let mut visited: HashMap<Slot, i32> = HashMap::new();
    let mut queue: VecDeque<(Slot, i32)> = VecDeque::new();
    queue.push_back(((0, 0, 0, 0), 0));

    while let Some((slot, value)) = queue.pop_front() {
        if deadends.contains(&slot) || visited.contains_key(&slot) {
            continue;
        }

        visited.insert(slot, value);

        let offsets = [
            (1, 0, 0, 0),
            (-1, 0, 0, 0),
            (0, 1, 0, 0),
            (0, -1, 0, 0),
            (0, 0, 1, 0),
            (0, 0, -1, 0),
            (0, 0, 0, 1),
            (0, 0, 0, -1),
        ];

        for offset in offsets {
            let new_slot = (
                (slot.0 + (10 + offset.0) as u8) % 10,
                (slot.1 + (10 + offset.1) as u8) % 10,
                (slot.2 + (10 + offset.2) as u8) % 10,
                (slot.3 + (10 + offset.3) as u8) % 10,
            );
            queue.push_back((new_slot, value + 1));
        }
    }

    match visited.get(&target) {
        Some(value) => *value,
        None => -1,
    }
}

fn to_slot(s: &str) -> Slot {
    let slots = s.chars().collect::<Vec<char>>();
    (
        slots[0] as u8 - '0' as u8,
        slots[1] as u8 - '0' as u8,
        slots[2] as u8 - '0' as u8,
        slots[3] as u8 - '0' as u8,
    )
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve(HashSet::from([(1, 1, 1, 1)]), (1, 0, 0, 0)), 1);
    }
}
