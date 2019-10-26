#[cfg(test)]
mod test {
    use crate::respond::parse;
    use crate::respond::token;
    use std::borrow::Cow;

    #[test]
    fn token_space() {
        macro_rules! assert_tokens {
            ($m:expr, $t1:expr, $t2:expr $(,)?) => {
                assert_eq!(
                    token::transform($m),
                    Some(vec![$t1("foo"), $t2("bar")]),
                )
            };
            ($m:expr, $t1:expr, $t2:expr, $t3:expr $(,)?) => {
                assert_eq!(
                    token::transform($m),
                    Some(vec![$t1("foo"), $t2, $t3("bar")]),
                )
            };
        }
        assert_tokens!("foo bar", token::Token::Arg, token::Token::Arg);
        assert_tokens!("foo\nbar", token::Token::Arg, token::Token::Arg);
        assert_tokens!("foo bar ", token::Token::Arg, token::Token::Arg);
        assert_tokens!(" foo bar ", token::Token::Arg, token::Token::Arg);
        assert_tokens!("!foo bar", token::Token::Fn, token::Token::Arg);
        assert_tokens!("!foo\nbar", token::Token::Fn, token::Token::Arg);
        assert_tokens!("!foo bar ", token::Token::Fn, token::Token::Arg);
        assert_tokens!(" !foo bar ", token::Token::Fn, token::Token::Arg);
        assert_tokens!("!foo !bar", token::Token::Fn, token::Token::Fn);
        assert_tokens!("!foo\n!bar", token::Token::Fn, token::Token::Fn);
        assert_tokens!("!foo !bar ", token::Token::Fn, token::Token::Fn);
        assert_tokens!(" !foo !bar ", token::Token::Fn, token::Token::Fn);
    }

    #[test]
    fn token_empty() {
        assert_eq!(token::transform(""), None);
        assert_eq!(token::transform("\"\""), None);
    }

    #[test]
    fn token_quotation() {
        assert_eq!(
            token::transform("\"foo bar\""),
            Some(vec![token::Token::Arg("foo bar")]),
        );
        assert_eq!(
            token::transform("\'foo bar\'"),
            Some(vec![token::Token::Arg("foo bar")]),
        );
        assert_eq!(token::transform("\'foo bar"), None);
        assert_eq!(token::transform("\"foo bar"), None);
        assert_eq!(
            token::transform("\"foo\nbar\""),
            Some(vec![token::Token::Arg("foo\nbar")]),
        );
        assert_eq!(
            token::transform("\'foo\nbar\'"),
            Some(vec![token::Token::Arg("foo\nbar")]),
        );
        assert_eq!(
            token::transform("\"foo\"bar"),
            Some(vec![token::Token::Arg("foo"), token::Token::Arg("bar")]),
        );
    }

    #[test]
    fn parse_echo() {
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("ECHO"),
                token::Token::Arg("foo"),
            ]),
            Some(Cow::from("foo")),
        );
        assert_eq!(
            parse::transform(
                vec![token::Token::Fn("ECHO"), token::Token::Arg("foo")]
                    .as_slice()
            ),
            Some(Cow::from("foo")),
        );
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("ECHO"),
                token::Token::Arg("foo bar"),
            ]),
            Some(Cow::from("foo bar")),
        );
    }

    #[test]
    fn parse_join() {
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("JOIN"),
                token::Token::Arg("foo"),
                token::Token::Arg("bar"),
            ]),
            Some(Cow::from("foo bar")),
        );
    }

    #[test]
    fn parse_rev() {
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("REV"),
                token::Token::Arg("foo bar baz"),
            ]),
            Some(Cow::from("zab rab oof")),
        );
    }

    #[test]
    fn parse_ban() {
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("ban"),
                token::Token::Arg("foo bar"),
            ]),
            Some(Cow::from(
                ":robot_face::no_entry: \
                 foo bar has been banned \
                 :no_entry::robot_face:"
            )),
        );
    }

    #[test]
    fn parse_bday() {
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("bday"),
                token::Token::Arg("foo bar"),
            ]),
            Some(Cow::from(
                ":robot_face::birthday: \
                 Happy birthday, foo bar! \
                 :birthday::robot_face:"
            )),
        );
    }

    #[test]
    fn parse_welcome() {
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("welcome"),
                token::Token::Arg("foo bar"),
            ]),
            Some(Cow::from(
                ":robot_face::open_hands: \
                 Welcome, foo bar! \
                 :open_hands::robot_face:"
            )),
        );
    }

    #[test]
    fn parse_savage() {
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("savage"),
                token::Token::Arg("foo bar"),
            ]),
            Some(Cow::from(
                ":robot_face::savage: foo bar is like a single grain of sand \
                 in the Sahara desert that is Macho Madness \
                 :savage::robot_face:"
            )),
        );
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("savage"),
                token::Token::Arg("foo bars"),
            ]),
            Some(Cow::from(
                ":robot_face::savage: foo bars are like a single grain of \
                 sand in the Sahara desert that is Macho Madness \
                 :savage::robot_face:"
            )),
        );
    }

    #[test]
    fn parse_invalid_fn() {
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("foo"),
                token::Token::Arg("bar"),
            ]),
            None,
        )
    }

    #[test]
    fn parse_invalid_arg() {
        assert_eq!(parse::transform(&[token::Token::Arg("foo")]), None)
    }
}
