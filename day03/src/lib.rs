use anyhow::Result;

enum Token {
    None,
    Mul,
    OpenParen,
    Digit,
    Comma,
    DoState,
}
// TODO rewrite all this to a struct
pub fn part_1(input: &str) -> Result<i64> {
    let mut vec_buffer = vec![];
    let mut prev = ' ';
    let mut token = Token::None;
    let mut buffer_1: Vec<char> = vec![];
    let mut buffer_2: Vec<char> = vec![];
    let mut post_comma = false;
    input
        .chars()
        .map(|c| {
            match token {
                Token::None => {
                    if c == 'm' {
                        token = Token::Mul;
                    }
                }
                Token::Mul => match &c {
                    'u' => {
                        if prev != 'm' {
                            token = Token::None
                        }
                    }
                    'l' => {
                        if prev != 'u' {
                            token = Token::None
                        } else {
                            token = Token::OpenParen
                        }
                    }
                    _ => token = Token::None,
                },
                Token::OpenParen => {
                    if c != '(' {
                        token = Token::None
                    } else {
                        token = Token::Digit
                    }
                }
                Token::Digit => {
                    if c == ',' {
                        post_comma = true;
                        token = Token::Comma;
                    } else if c == ')' {
                        // end mul wrapper
                        let first = buffer_1.iter().collect::<String>().parse::<i64>().unwrap();
                        let second = buffer_2.iter().collect::<String>().parse::<i64>().unwrap();
                        buffer_1 = vec![];
                        buffer_2 = vec![];
                        post_comma = false;
                        token = Token::None;
                        vec_buffer.push(first * second);
                    } else if c.is_ascii_digit() {
                        if post_comma {
                            buffer_2.push(c);
                        } else {
                            buffer_1.push(c);
                        }
                    } else {
                        buffer_1 = vec![];
                        buffer_2 = vec![];
                        post_comma = false;
                        token = Token::None
                    }
                }
                Token::Comma => {
                    if c.is_ascii_digit() {
                        buffer_2.push(c);
                        token = Token::Digit
                    } else {
                        buffer_1 = vec![];
                        buffer_2 = vec![];
                        post_comma = false;
                        token = Token::None
                    }
                }
                _ => unreachable!(),
            };
            prev = c;
        })
        .count();
    let mul = vec_buffer.iter().sum();
    Ok(mul)
}

pub fn part_2(input: &str) -> Result<i64> {
    let mut exec = true;
    let mut is_dont = false;

    // <C-c> + <C-v> with a toggle state
    let mut vec_buffer = vec![];
    let mut prev = ' ';
    let mut token = Token::None;
    let mut buffer_1: Vec<char> = vec![];
    let mut buffer_2: Vec<char> = vec![];
    let mut post_comma = false;
    input
        .chars()
        .map(|c| {
            match token {
                Token::None => match &c {
                    'm' => {
                        if exec {
                            token = Token::Mul;
                        }
                    }
                    'd' => token = Token::DoState,
                    _ => {}
                },
                Token::Mul => match &c {
                    'u' => {
                        if prev != 'm' {
                            token = Token::None
                        }
                    }
                    'l' => {
                        if prev != 'u' {
                            token = Token::None
                        } else {
                            token = Token::OpenParen
                        }
                    }
                    _ => token = Token::None,
                },
                Token::OpenParen => {
                    if c != '(' {
                        token = Token::None
                    } else {
                        token = Token::Digit
                    }
                }
                Token::Digit => {
                    if c == ',' {
                        post_comma = true;
                        token = Token::Comma;
                    } else if c == ')' {
                        // end mul wrapper
                        let first = buffer_1.iter().collect::<String>().parse::<i64>().unwrap();
                        let second = buffer_2.iter().collect::<String>().parse::<i64>().unwrap();
                        buffer_1 = vec![];
                        buffer_2 = vec![];
                        post_comma = false;
                        token = Token::None;
                        vec_buffer.push(first * second);
                    } else if c.is_ascii_digit() {
                        if post_comma {
                            buffer_2.push(c);
                        } else {
                            buffer_1.push(c);
                        }
                    } else {
                        buffer_1 = vec![];
                        buffer_2 = vec![];
                        post_comma = false;
                        token = Token::None
                    }
                }
                Token::Comma => {
                    if c.is_ascii_digit() {
                        buffer_2.push(c);
                        token = Token::Digit
                    } else {
                        buffer_1 = vec![];
                        buffer_2 = vec![];
                        post_comma = false;
                        token = Token::None
                    }
                }
                Token::DoState => match &c {
                    'o' => {
                        if prev != 'd' {
                            token = Token::None;
                        }
                    }
                    'n' => {
                        if prev != 'o' {
                            token = Token::None;
                        } else {
                            is_dont = true;
                        }
                    }
                    '\'' => {
                        if prev != 'n' {
                            token = Token::None;
                            is_dont = false;
                        }
                    }
                    't' => {
                        if prev != '\'' {
                            token = Token::None;
                            is_dont = false;
                        }
                    }
                    '(' => {
                        if !(prev == 't' || prev == 'o') {
                            token = Token::None;
                            is_dont = false;
                        }
                    }
                    ')' => {
                        if prev != '(' {
                            token = Token::None;
                            is_dont = false;
                        } else {
                            exec = !is_dont;
                            is_dont = false;
                            token = Token::None;
                        }
                    }
                    _ => token = Token::None,
                },
            };
            prev = c;
        })
        .count();
    let mul = vec_buffer.iter().sum();
    Ok(mul)
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use anyhow::Result;

    #[test]
    fn test_day() -> Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_1(input)?, 161);
        Ok(())
    }

    #[test]
    fn test_day_2() -> Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(input)?, 48);
        Ok(())
    }
}
