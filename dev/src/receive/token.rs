use std::iter::Enumerate;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    Quotation(&'a str),
    Literal(&'a str),
}

pub fn transform(blob: &str) -> Option<Vec<Token>> {
    if blob.is_empty() {
        return None;
    }
    let mut stack: Vec<Token> = Vec::with_capacity(blob.len());
    let mut chars: Enumerate<Chars> = blob.chars().enumerate();
    macro_rules! capture {
        ($t:expr, $i:expr, $j:expr $(,)?) => {
            stack.push($t(&blob[$i..$j]));
            break;
        };
        ($t1:expr, $i:expr, $j:expr, $t2:expr $(,)?) => {
            stack.push($t1(&blob[$i..$j]));
            stack.push($t2);
            break;
        };
    }
    loop {
        if let Some((i, c)) = chars.next() {
            match c {
                ' ' | '\n' => (),
                '\\' => return None,
                '{' => stack.push(Token::OpenBracket),
                '}' => stack.push(Token::CloseBracket),
                ':' => stack.push(Token::Colon),
                ',' => stack.push(Token::Comma),
                '\"' => loop {
                    if let Some((j, c)) = chars.next() {
                        match c {
                            '\\' => {
                                chars.next();
                            }
                            '\"' => {
                                capture!(Token::Quotation, i + 1, j);
                            }
                            _ => (),
                        }
                    } else {
                        return Some(stack);
                    }
                },
                _ => loop {
                    if let Some((j, c)) = chars.next() {
                        match c {
                            ' ' | '\n' => {
                                capture!(Token::Literal, i, j);
                            }
                            '}' => {
                                capture!(
                                    Token::Literal,
                                    i,
                                    j,
                                    Token::CloseBracket,
                                );
                            }
                            ',' => {
                                capture!(Token::Literal, i, j, Token::Comma);
                            }
                            _ => (),
                        }
                    } else {
                        return Some(stack);
                    }
                },
            }
        } else {
            return Some(stack);
        }
    }
}
