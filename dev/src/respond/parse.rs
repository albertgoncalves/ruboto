use crate::respond::token::Token;
use std::borrow::Cow;
use std::env;

fn rev(arg: &str) -> Cow<'_, str> {
    Cow::from(arg.chars().rev().collect::<String>())
}

macro_rules! combine_3 {
    ($a:expr, $b:expr, $c:expr $(,)?) => {{
        let mut payload: String =
            String::with_capacity($a.len() + $b.len() + $c.len());
        payload.push_str($a);
        payload.push_str($b);
        payload.push_str($c);
        payload
    }};
}

fn join<'a>(left: &'a str, right: &'a str) -> Cow<'a, str> {
    Cow::from(combine_3!(left, " ", right))
}

fn ban(arg: &str) -> Cow<'_, str> {
    Cow::from(combine_3!(
        ":robot_face::no_entry: ",
        arg,
        " has been banned :no_entry::robot_face:",
    ))
}

fn bday(arg: &str) -> Cow<'_, str> {
    Cow::from(combine_3!(
        ":robot_face::birthday: Happy birthday, ",
        arg,
        "! :birthday::robot_face:",
    ))
}

fn welcome(arg: &str) -> Cow<'_, str> {
    Cow::from(combine_3!(
        ":robot_face::open_hands: Welcome, ",
        arg,
        "! :open_hands::robot_face:",
    ))
}

fn savage(arg: &str) -> Cow<'_, str> {
    Cow::from(combine_3!(
        ":robot_face::savage: ",
        arg,
        " is like a single grain of sand in the Sahara desert that is \
         Macho Madness :savage::robot_face:",
    ))
}

pub fn transform<'a>(tokens: &[Token<'a>]) -> Option<Cow<'a, str>> {
    match tokens {
        [Token::Fn("ECHO"), Token::Arg(arg)] => Some(Cow::from(*arg)),
        [Token::Fn("REV"), Token::Arg(arg)] => Some(rev(arg)),
        [Token::Fn("JOIN"), Token::Arg(left), Token::Arg(right)] => {
            Some(join(left, right))
        }
        [Token::Fn("WORD")] => Some(Cow::from("expediency")),
        [Token::Fn("ban"), Token::Arg(arg)] => Some(ban(arg)),
        [Token::Fn("bday"), Token::Arg(arg)] => Some(bday(arg)),
        [Token::Fn("welcome"), Token::Arg(arg)] => Some(welcome(arg)),
        [Token::Fn("savage"), Token::Arg(arg)] => Some(savage(arg)),
        [Token::Fn("vaporwave")] => env::var("VAPORWAVE").ok().map(Cow::from),
        [Token::Fn("bernar")] => env::var("BERNAR").ok().map(Cow::from),
        _ => None,
    }
}
