use crate::respond::token;
use std::borrow;

#[allow(clippy::type_complexity)]
pub fn transform<'a>(
    tokens: &[token::Token<'a>],
) -> Option<borrow::Cow<'a, str>> {
    let n: usize = tokens.len();
    let mut stack: Vec<Box<dyn Fn(borrow::Cow<str>) -> borrow::Cow<str>>> =
        Vec::with_capacity(n);
    let mut response: Option<borrow::Cow<str>> = None;
    let mut i: usize = 0;
    macro_rules! break_if {
        () => {
            i += 1;
            if i == n {
                break;
            }
        };
    }
    macro_rules! join {
        ($l:expr) => {
            stack.push(Box::new(move |right: borrow::Cow<str>| {
                let mut left: String = $l.to_owned();
                left.push_str(" ");
                left.push_str(&right);
                borrow::Cow::from(left)
            }));
        };
    }
    loop {
        match tokens[i] {
            token::Token::Fn("echo") => {
                stack.push(Box::new(|arg: borrow::Cow<str>| arg))
            }
            token::Token::Fn("rev") => {
                stack.push(Box::new(|arg: borrow::Cow<str>| {
                    borrow::Cow::from(arg.chars().rev().collect::<String>())
                }))
            }
            token::Token::Fn("join") => {
                if (i + 1) <= n {
                    match tokens[i + 1] {
                        token::Token::Arg(arg) => {
                            join!(arg);
                            break_if!();
                        }
                        _ => return None,
                    }
                }
            }
            token::Token::Arg(arg) => {
                if response == None {
                    response = Some(borrow::Cow::from(arg))
                } else {
                    return None;
                }
            }
            token::Token::Fn(_) => return None,
        };
        break_if!();
    }
    for f in stack {
        response = response.map(f);
    }
    response
}
