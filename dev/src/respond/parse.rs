use crate::respond::token::Token;
use std::borrow::Cow;

fn rev(arg: &str) -> Cow<'_, str> {
    Cow::from(arg.chars().rev().collect::<String>())
}

fn combine_3(a: &str, b: &str, c: &str) -> String {
    let mut payload: String =
        String::with_capacity(a.len() + b.len() + c.len());
    payload.push_str(a);
    payload.push_str(b);
    payload.push_str(c);
    payload
}

fn join<'a>(left: &'a str, right: &'a str) -> Cow<'a, str> {
    Cow::from(combine_3(left, " ", right))
}

fn ban(arg: &str) -> Cow<'_, str> {
    Cow::from(combine_3(
        ":robot_face::no_entry: ",
        arg,
        " has been banned :no_entry::robot_face:",
    ))
}

fn bday(arg: &str) -> Cow<'_, str> {
    Cow::from(combine_3(
        ":robot_face::birthday: Happy birthday, ",
        arg,
        "! :birthday::robot_face:",
    ))
}

fn welcome(arg: &str) -> Cow<'_, str> {
    Cow::from(combine_3(
        ":robot_face::open_hands: Welcome, ",
        arg,
        "! :open_hands::robot_face:",
    ))
}

fn savage(arg: &str) -> Cow<'_, str> {
    if arg.ends_with('s') {
        Cow::from(combine_3(
            ":robot_face::savage: ",
            arg,
            " are like a single grain of sand in the Sahara desert that is \
             Macho Madness :savage::robot_face:",
        ))
    } else {
        Cow::from(combine_3(
            ":robot_face::savage: ",
            arg,
            " is like a single grain of sand in the Sahara desert that is \
             Macho Madness :savage::robot_face:",
        ))
    }
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
        _ => None,
    }
}
