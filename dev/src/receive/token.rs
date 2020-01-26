#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    OpenBrace,
    CloseBrace,
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
    let n: usize = blob.len();
    let mut stack: Vec<Token> = Vec::with_capacity(n);
    let chars: &[u8] = blob.as_bytes();
    let mut i: usize = 0;
    loop {
        if n <= i {
            return Some(stack);
        }
        match chars[i] as char {
            ' ' | '\n' => (),
            '\\' => return None,
            '{' => stack.push(Token::OpenBrace),
            '}' => stack.push(Token::CloseBrace),
            ':' => stack.push(Token::Colon),
            ',' => stack.push(Token::Comma),
            '[' => stack.push(Token::OpenBracket),
            ']' => stack.push(Token::CloseBracket),
            '\"' => {
                let mut j: usize = i;
                loop {
                    j += 1;
                    if j < n {
                        match chars[j] as char {
                            '\\' => {
                                j += 1;
                            }
                            '\"' => {
                                stack
                                    .push(Token::Quotation(&blob[(i + 1)..j]));
                                i = j;
                                break;
                            }
                            _ => (),
                        }
                    } else {
                        return Some(stack);
                    }
                }
            }
            _ => {
                let mut j: usize = i;
                loop {
                    j += 1;
                    if j < n {
                        match chars[j] as char {
                            ' ' | '\n' => {
                                stack.push(Token::Literal(&blob[i..j]));
                                i = j;
                                break;
                            }
                            '}' => {
                                stack.push(Token::Literal(&blob[i..j]));
                                stack.push(Token::CloseBrace);
                                i = j;
                                break;
                            }
                            ',' => {
                                stack.push(Token::Literal(&blob[i..j]));
                                stack.push(Token::Comma);
                                i = j;
                                break;
                            }
                            _ => (),
                        }
                    } else {
                        return Some(stack);
                    }
                }
            }
        }
        i += 1;
    }
}
