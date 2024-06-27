use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::io::{self};

// 2^32 = (2^10)^3 * 4 ~= 10^9 * 4
fn main() -> io::Result<()> {
    let mut buf = String::new();
    let mut hands = vec![];

    while io::stdin().read_line(&mut buf)? != 0 {
        hands.push(Hand::from_string(buf.trim()));
        buf.clear();
    }

    hands.sort();

    let winnings = hands.iter().enumerate().fold(0, |winnings, (i, hand)| {
        (i as i32 + 1) * hand.bid + winnings
    });
    println!("{winnings}");

    Ok(())
}

#[derive(Debug)]
struct Hand {
    cards: Vec<i32>,
    strength: i32,
    bid: i32,
}

impl Hand {
    fn from_string(s: &str) -> Self {
        // 32T3K 765
        let (s0, s1) = s.split_once(' ').unwrap();

        let mut cards = s0
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                _ => c as i32 - '0' as i32,
            })
            .collect::<Vec<i32>>();

        let strength = Self::calculate_strength(&cards);

        Self {
            cards,
            strength,
            bid: s1.parse::<i32>().unwrap(),
        }
    }

    fn calculate_strength(cards: &[i32]) -> i32 {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        cards.iter().for_each(|c| {
            counts.insert(*c, 1 + counts.get(c).unwrap_or(&0));
        });

        if let Some(j_count) = counts.remove(&0) {
            let (&k, &v) = counts
                .iter()
                .reduce(|acc, x| if acc.1 > x.1 { acc } else { x })
                .unwrap_or((&0, &0));
            counts.insert(k, v + j_count);
        }

        if counts.len() == 1 {
            return 7;
        }

        if counts.len() == 2 {
            return if counts.iter().any(|(_, &v)| v == 4) {
                6
            } else {
                5
            };
        }

        if counts.len() == 3 {
            return if counts.iter().any(|(_, &v)| v == 3) {
                4
            } else {
                3
            };
        }

        return if counts.len() == 4 { 2 } else { 1 };
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength > other.strength {
            Ordering::Greater
        } else if self.strength < other.strength {
            Ordering::Less
        } else {
            for i in 0..5 {
                if self.cards[i] == other.cards[i] {
                    continue;
                }

                return if self.cards[i] > other.cards[i] {
                    Ordering::Greater
                } else {
                    Ordering::Less
                };
            }
            Ordering::Equal
        }
    }
}
