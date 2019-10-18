use crate::respond::token::Token;
use std::borrow::Cow;

#[allow(clippy::type_complexity)]
pub fn transform<'a>(tokens: &[Token<'a>]) -> Option<Cow<'a, str>> {
    match tokens {
        [Token::Fn("echo"), Token::Arg(arg)] => Some(Cow::from(*arg)),
        [Token::Fn("rev"), Token::Arg(arg)] => {
            Some(Cow::from(arg.chars().rev().collect::<String>()))
        }
        [Token::Fn("join"), Token::Arg(left), Token::Arg(right)] => {
            let mut l: String = left.to_string();
            l.push_str(" ");
            l.push_str(right);
            Some(Cow::from(l))
        }
        _ => None,
    }
}
