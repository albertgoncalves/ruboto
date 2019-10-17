#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Command(&'a str),
    Content(&'a str),
}

pub fn transform(message: &str) -> Option<Vec<Token>> {
    if message.is_empty() {
        return None;
    }
    let n: usize = message.len();
    let k: usize = (n / 2) + 1;
    let mut stack: Vec<Token> = Vec::with_capacity(k);
    let mut chars = message.chars().enumerate();
    macro_rules! capture {
        ($t:expr, $i:expr $(,)?) => {
            loop {
                if let Some((j, c)) = chars.next() {
                    match c {
                        ' ' | '\n' => {
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
    loop {
        if let Some((i, c)) = chars.next() {
            match c {
                ' ' | '\n' => (),
                '!' => capture!(Token::Command, i + 1),
                _ => capture!(Token::Content, i),
            }
        } else {
            return Some(stack);
        }
    }
}
