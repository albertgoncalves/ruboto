#[cfg(test)]
mod test {
    use crate::respond::parse;
    use crate::respond::token;
    use std::borrow;

    #[test]
    fn token_spaces() {
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
    fn token_echo() {
        assert_eq!(
            parse::transform(&[
                token::Token::Fn("echo"),
                token::Token::Arg("foo"),
            ]),
            Some(borrow::Cow::from("foo")),
        );
        assert_eq!(
            parse::transform(
                vec![token::Token::Fn("echo"), token::Token::Arg("foo")]
                    .as_slice()
            ),
            Some(borrow::Cow::from("foo")),
        );
    }

    #[test]
    fn token_echo_rev() {
        macro_rules! assert_transform {
            ($a:expr, $b:expr, $c:expr $(,)?) => {
                assert_eq!(
                    parse::transform(&[$a, $b, $c]),
                    Some(borrow::Cow::from("oof")),
                )
            };
        }
        assert_transform!(
            token::Token::Arg("foo"),
            token::Token::Fn("echo"),
            token::Token::Fn("rev"),
        );
        assert_transform!(
            token::Token::Fn("echo"),
            token::Token::Arg("foo"),
            token::Token::Fn("rev"),
        );
        assert_transform!(
            token::Token::Fn("echo"),
            token::Token::Fn("rev"),
            token::Token::Arg("foo"),
        );
        assert_transform!(
            token::Token::Fn("rev"),
            token::Token::Fn("echo"),
            token::Token::Arg("foo"),
        );
    }

    #[test]
    fn token_join() {
        macro_rules! assert_transform {
            ($a:expr, $b:expr, $c:expr $(,)?) => {
                assert_eq!(
                    parse::transform(&[$a, $b, $c]),
                    Some(borrow::Cow::from("foo bar")),
                )
            };
        }
        assert_transform!(
            token::Token::Fn("join"),
            token::Token::Arg("foo"),
            token::Token::Arg("bar"),
        );
        assert_transform!(
            token::Token::Arg("bar"),
            token::Token::Fn("join"),
            token::Token::Arg("foo"),
        );
    }
}
