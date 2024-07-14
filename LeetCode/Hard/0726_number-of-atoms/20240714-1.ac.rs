use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Token {
    LeftParenthesis,
    RightParenthesis(i32),
    Atom { symbol: String, count: i32 },
}

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        // K4(ON(SO3)2)2
        // (((K4))1)
        // ((K4)2(K)(Z))

        let tokens = analyze_formula(formula.chars().collect());
        let counts: HashMap<String, i32> = analyze_tokens(tokens);

        let mut outs = vec![];
        for (s, count) in counts {
            outs.push((s, count));
        }
        outs.sort();

        let mut ans = "".to_string();
        for (s, count) in outs {
            ans.push_str(&s);
            if count > 1 {
                ans.push_str(count.to_string().as_str());
            }
        }

        ans
    }
}

fn analyze_tokens(tokens: Vec<Token>) -> HashMap<String, i32> {
    let mut stack: VecDeque<Token> = VecDeque::new();

    for token in tokens {
        match token {
            Token::RightParenthesis(count) => {
                let mut tmp: Vec<(String, i32)> = vec![];
                while let Some(token) = stack.pop_back() {
                    match token {
                        Token::LeftParenthesis => {
                            break;
                        }
                        Token::RightParenthesis(_) => {
                            unreachable!();
                        }
                        Token::Atom { symbol, count: c } => {
                            tmp.push((symbol, c * count));
                        }
                    }
                }

                while let Some((symbol, count)) = tmp.pop() {
                    stack.push_back(Token::Atom { symbol, count })
                }
            }
            _ => {
                stack.push_back(token);
            }
        }
    }

    let mut counts: HashMap<String, i32> = HashMap::new();
    while let Some(token) = stack.pop_front() {
        match token {
            Token::Atom { symbol, count } => {
                let v = counts.get(&symbol).unwrap_or(&0) + count;
                counts.insert(symbol, v);
            }
            _ => unreachable!(),
        }
    }

    counts
}

fn analyze_formula(formula: Vec<char>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for i in 0..formula.len() {
        let c = formula[i];
        let prev_c = if i > 0 { Some(formula[i - 1]) } else { None };

        if c.is_ascii_uppercase() {
            tokens.push(Token::Atom {
                symbol: String::from(c),
                count: 1,
            });
        } else if c == '(' {
            tokens.push(Token::LeftParenthesis);
        } else if c == ')' {
            tokens.push(Token::RightParenthesis(1));
        } else if c.is_ascii_lowercase() {
            match tokens.last_mut() {
                Some(Token::Atom { symbol, count: _ }) => symbol.push(c),
                _ => unreachable!(),
            }
        } else {
            assert!(c.is_ascii_digit());
            let digit = c as i32 - '0' as i32;

            match tokens.last_mut() {
                Some(Token::Atom { symbol: _, count }) => {
                    *count = if prev_c.unwrap().is_ascii_digit() {
                        *count * 10 + digit
                    } else {
                        digit
                    };
                }
                Some(Token::RightParenthesis(count)) => {
                    *count = if prev_c.unwrap().is_ascii_digit() {
                        *count * 10 + digit
                    } else {
                        digit
                    };
                }
                _ => unreachable!(),
            }
        }
    }

    tokens
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {}
}
