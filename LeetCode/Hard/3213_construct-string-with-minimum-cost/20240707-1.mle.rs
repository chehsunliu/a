use std::cmp::min;
use std::collections::HashMap;

type Dict<'a> = HashMap<char, HashMap<&'a str, i32>>;

impl Solution {
    pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
        let mut dict: Dict = HashMap::new();
        for i in 0..words.len() {
            let word = words[i].as_str();
            let cost = costs[i];
            let c = word.chars().next().unwrap();

            if let Some(c_dict) = dict.get_mut(&c) {
                if let Some(item) = c_dict.get_mut(word) {
                    *item = min(*item, cost);
                } else {
                    c_dict.insert(word, cost);
                }
            } else {
                dict.insert(c, HashMap::from([(word, cost)]));
            }
        }

        f(String::new(), 0, &target, &dict, &mut HashMap::new())
    }
}

fn f(s: String, previous_cost: i32, t: &str, dict: &Dict, dp: &mut HashMap<String, i32>) -> i32 {
    if s == t {
        return previous_cost;
    }

    if s.len() > t.len() {
        return -1;
    }

    if s != &t[..s.len()] {
        return -1;
    }

    let next_c = t[s.len()..].chars().next().unwrap();
    if !dict.contains_key(&next_c) {
        return -1;
    }

    if let Some(dp_min_cost) = dp.get(&s) {
        return if *dp_min_cost == -1 {
            -1
        } else {
            *dp_min_cost + previous_cost
        };
    }

    let mut min_cost = -1;

    let c_dict = dict.get(&next_c).unwrap();
    for (&word, &word_cost) in c_dict {
        let mut next_s = s.clone();
        next_s.push_str(word);
        let f_cost = f(next_s, word_cost, t, dict, dp);
        if min_cost != -1 && f_cost != -1 {
            min_cost = min(min_cost, f_cost);
        } else if min_cost == -1 && f_cost != -1 {
            min_cost = f_cost;
        }
    }

    dp.insert(s.clone(), min_cost);

    if min_cost == -1 {
        -1
    } else {
        min_cost + previous_cost
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let target = "xmjpkowhuefntanqvygzukqlxrrpsbuscjesdxilsjodkjjzhfrzymiaeedjsfpquiu";
        let words = vec![
            "u",
            "m",
            "i",
            "d",
            "busc",
            "z",
            "j",
            "s",
            "huefntanqvygzuk",
            "d",
            "qlxrrpsbuscjesdxilsjodkjj",
            "nqvygzukqlxrrpsbuscjesdxilsjodkjjzhfrzymiaee",
            "h",
            "c",
            "o",
            "s",
            "l",
            "sjodkjjzhfrzymi",
            "aeedjsfpqu",
            "i",
            "u",
            "kowhue",
            "pkowhuefntanqvygzukqlxrrpsbus",
            "x",
            "j",
            "djsfpquiu",
            "dxilsjodkjjzhfrzymiaeedjsfpqu",
            "j",
            "jodkjjzhfrzymia",
            "xrrpsbuscjesdxilsj",
            "r",
            "f",
            "iu",
            "k",
            "frzymiaeedjsf",
            "zymiaee",
            "djsf",
            "j",
            "mjpkowh",
            "d",
            "j",
            "x",
            "i",
            "miaeed",
            "fntanqvygzukqlxrrpsbuscjesdxilsjodkjjzhfrzymiaeedjsfpq",
            "lxrrpsbuscjesdxilsjod",
            "jsfpqu",
            "e",
        ];
        let costs = vec![
            41, 2, 23, 44, 1, 2, 31, 12, 4, 43, 17, 16, 19, 34, 36, 36, 16, 17, 30, 19, 34, 5, 43,
            28, 43, 31, 1, 21, 26, 2, 23, 10, 48, 24, 7, 44, 1, 46, 26, 25, 24, 31, 23, 13, 7, 13,
            24, 41,
        ];

        let target = target.to_string();
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let result = Solution::minimum_cost(target, words, costs);

        assert_eq!(result, 218);
    }

    #[test]
    fn basic2() {
        let target = "mooaiwrb";
        let words = vec!["o", "m", "wrb", "o", "ai"];
        let costs = vec![2, 1, 2, 1, 1];

        let target = target.to_string();
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let result = Solution::minimum_cost(target, words, costs);

        assert_eq!(result, 6);
    }

    #[test]
    fn basic3() {
        let target = "abcdef";
        let words = vec!["abdef", "abc", "d", "def", "ef"];
        let costs = vec![100, 1, 1, 10, 5];

        let target = target.to_string();
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let result = Solution::minimum_cost(target, words, costs);

        assert_eq!(result, 7);
    }

    #[test]
    fn basic4() {
        let target = "wodmw";
        let words = vec!["dm", "w", "o", "wo", "dmw"];
        let costs = vec![2, 3, 9, 6, 6];

        let target = target.to_string();
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let result = Solution::minimum_cost(target, words, costs);

        assert_eq!(result, 11);
    }
}
