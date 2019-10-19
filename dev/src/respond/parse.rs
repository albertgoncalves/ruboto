use crate::respond::token::Token;
use std::borrow::Cow;

pub fn transform<'a>(tokens: &[Token<'a>]) -> Option<Cow<'a, str>> {
    match tokens {
        [Token::Fn("echo"), Token::Arg(arg)] => Some(Cow::from(*arg)),
        [Token::Fn("rev"), Token::Arg(arg)] => {
            Some(Cow::from(arg.chars().rev().collect::<String>()))
        }
        [Token::Fn("join"), Token::Arg(left), Token::Arg(right)] => {
            let mut payload: String =
                String::with_capacity(left.len() + 1 + right.len());
            payload.push_str(left);
            payload.push_str(" ");
            payload.push_str(right);
            Some(Cow::from(payload))
        }
        _ => None,
    }
}
