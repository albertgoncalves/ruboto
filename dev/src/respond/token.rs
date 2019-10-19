use std::iter::Enumerate;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Fn(&'a str),
    Arg(&'a str),
}

#[allow(clippy::single_match)]
pub fn transform(message: &str) -> Option<Vec<Token>> {
    if message.is_empty() {
        return None;
    }
    let n: usize = message.len();
    let k: usize = (n / 2) + 1;
    let mut stack: Vec<Token> = Vec::with_capacity(k);
    let mut chars: Enumerate<Chars> = message.chars().enumerate();
    macro_rules! capture {
        ($t:expr, $i:expr, $d1:expr $(, $d2:expr)? $(,)?) => {
            loop {
                if let Some((j, c)) = chars.next() {
                    match c {
                        $d1 $(| $d2)? => {
                            stack.push($t(&message[$i..j]));
                            break;
                        }
                        _ => (),
                    }
                } else {
                    if $i != n {
                        stack.push($t(&message[$i..n]));
                    }
                    return Some(stack);
                }
            }
        };
    }
    macro_rules! capture_strict {
        ($t:expr, $i:expr, $d:expr $(,)?) => {
            loop {
                if let Some((j, c)) = chars.next() {
                    match c {
                        $d => {
                            stack.push($t(&message[$i..j]));
                            break;
                        }
                        _ => (),
                    }
                } else {
                    return None;
                }
            }
        };
    }
    loop {
        if let Some((i, c)) = chars.next() {
            match c {
                ' ' | '\n' => (),
                '\"' => capture_strict!(Token::Arg, i + 1, '\"'),
                '\'' => capture_strict!(Token::Arg, i + 1, '\''),
                '!' => capture!(Token::Fn, i + 1, ' ', '\n'),
                _ => capture!(Token::Arg, i, ' ', '\n'),
            }
        } else {
            return Some(stack);
        }
    }
}
