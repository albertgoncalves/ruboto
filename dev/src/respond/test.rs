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
                token::Token::Fn("echo"),
                token::Token::Arg("foo"),
            ]),
            Some(Cow::from("foo")),
        );
        assert_eq!(
            parse::transform(
                vec![token::Token::Fn("echo"), token::Token::Arg("foo")]
                    .as_slice()
            ),
            Some(Cow::from("foo")),
        );
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("echo"),
                token::Token::Arg("foo bar"),
            ]),
            Some(Cow::from("foo bar")),
        );
    }

    #[test]
    fn parse_join() {
        macro_rules! assert_transform {
            ($a:expr, $b:expr, $c:expr $(,)?) => {
                assert_eq!(
                    parse::transform(&[$a, $b, $c]),
                    Some(Cow::from("foo bar")),
                )
            };
        }
        assert_transform!(
            token::Token::Fn("join"),
            token::Token::Arg("foo"),
            token::Token::Arg("bar"),
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
