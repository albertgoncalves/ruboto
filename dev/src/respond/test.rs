#[cfg(test)]
mod test {
    use crate::respond::parse;
    use crate::respond::token;
    use crate::respond::token::Token;
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
        assert_tokens!("foo bar", Token::Arg, Token::Arg);
        assert_tokens!("foo\nbar", Token::Arg, Token::Arg);
        assert_tokens!("foo bar ", Token::Arg, Token::Arg);
        assert_tokens!(" foo bar ", Token::Arg, Token::Arg);
        assert_tokens!("!foo bar", Token::Fn, Token::Arg);
        assert_tokens!("!foo\nbar", Token::Fn, Token::Arg);
        assert_tokens!("!foo bar ", Token::Fn, Token::Arg);
        assert_tokens!(" !foo bar ", Token::Fn, Token::Arg);
        assert_tokens!("!foo !bar", Token::Fn, Token::Fn);
        assert_tokens!("!foo\n!bar", Token::Fn, Token::Fn);
        assert_tokens!("!foo !bar ", Token::Fn, Token::Fn);
        assert_tokens!(" !foo !bar ", Token::Fn, Token::Fn);
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
            Some(vec![Token::Arg("foo bar")]),
        );
        assert_eq!(
            token::transform("\'foo bar\'"),
            Some(vec![Token::Arg("foo bar")]),
        );
        assert_eq!(token::transform("\'foo bar"), None);
        assert_eq!(token::transform("\"foo bar"), None);
        assert_eq!(
            token::transform("\"foo\nbar\""),
            Some(vec![Token::Arg("foo\nbar")]),
        );
        assert_eq!(
            token::transform("\'foo\nbar\'"),
            Some(vec![Token::Arg("foo\nbar")]),
        );
        assert_eq!(
            token::transform("\"foo\"bar"),
            Some(vec![Token::Arg("foo"), Token::Arg("bar")]),
        );
    }

    #[test]
    fn parse_echo() {
        assert_eq!(
            parse::transform(&[Token::Fn("ECHO"), Token::Arg("foo"),]),
            Some(Cow::from("foo")),
        );
        assert_eq!(
            parse::transform(
                vec![Token::Fn("ECHO"), Token::Arg("foo")].as_slice()
            ),
            Some(Cow::from("foo")),
        );
        assert_eq!(
            parse::transform(&[Token::Fn("ECHO"), Token::Arg("foo bar"),]),
            Some(Cow::from("foo bar")),
        );
    }

    #[test]
    fn parse_join() {
        assert_eq!(
            parse::transform(&[
                Token::Fn("JOIN"),
                Token::Arg("foo"),
                Token::Arg("bar"),
            ]),
            Some(Cow::from("foo bar")),
        );
    }

    #[test]
    fn parse_rev() {
        assert_eq!(
            parse::transform(&[Token::Fn("REV"), Token::Arg("foo bar baz"),]),
            Some(Cow::from("zab rab oof")),
        );
    }

    #[test]
    fn parse_ban() {
        assert_eq!(
            parse::transform(&[Token::Fn("ban"), Token::Arg("foo bar"),]),
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
            parse::transform(&[Token::Fn("bday"), Token::Arg("foo bar"),]),
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
            parse::transform(&[Token::Fn("welcome"), Token::Arg("foo bar"),]),
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
            parse::transform(&[Token::Fn("savage"), Token::Arg("foo bar"),]),
            Some(Cow::from(
                ":robot_face::savage: foo bar is like a single grain of sand \
                 in the Sahara desert that is Macho Madness \
                 :savage::robot_face:"
            )),
        );
    }

    #[test]
    fn parse_invalid_fn() {
        assert_eq!(
            parse::transform(&[Token::Fn("foo"), Token::Arg("bar"),]),
            None,
        )
    }

    #[test]
    fn parse_invalid_arg() {
        assert_eq!(parse::transform(&[Token::Arg("foo")]), None)
    }
}
