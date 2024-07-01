use std::collections::VecDeque;

#[derive(Debug)]
enum Token {
    Repeat(i32),
    Chars(String),
    LeftSquare,
}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: VecDeque<Token> = VecDeque::new();

        for c in s.chars() {
            match c {
                '0'..='9' => handle_numbers(c, &mut stack),
                '[' => handle_left_square(&mut stack),
                ']' => handle_right_square(c, &mut stack),
                _ => handle_chars(c, &mut stack),
            }
        }

        match stack.pop_back().unwrap() {
            Token::Chars(chars) => chars,
            _ => unreachable!(),
        }
    }
}

fn handle_numbers(c: char, stack: &mut VecDeque<Token>) {
    let v = c as i32 - '0' as i32;
    if let Some(Token::Repeat(old_v)) = stack.back_mut() {
        *old_v = *old_v * 10 + v;
    } else {
        stack.push_back(Token::Repeat(v));
    }
}

fn handle_left_square(stack: &mut VecDeque<Token>) {
    stack.push_back(Token::LeftSquare);
}

fn handle_right_square(c: char, stack: &mut VecDeque<Token>) {
    let chars = match stack.pop_back().unwrap() {
        Token::Chars(chars) => chars,
        _ => unreachable!(),
    };

    match stack.pop_back().unwrap() {
        Token::LeftSquare => {}
        _ => unreachable!(),
    };

    let count = match stack.pop_back().unwrap() {
        Token::Repeat(count) => count,
        _ => unreachable!(),
    };

    let new_chars = chars.repeat(count as usize);
    if let Some(Token::Chars(chars)) = stack.back_mut() {
        chars.push_str(&new_chars);
    } else {
        stack.push_back(Token::Chars(new_chars));
    }
}

fn handle_chars(c: char, stack: &mut VecDeque<Token>) {
    if let Some(Token::Chars(chars)) = stack.back_mut() {
        chars.push(c);
    } else {
        stack.push_back(Token::Chars(c.to_string()));
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc");
        assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc");
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef"
        );
    }
}
