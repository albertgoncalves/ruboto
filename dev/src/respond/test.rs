#[cfg(test)]
mod test {
    use crate::respond::token;

    #[test]
    fn token() {
        macro_rules! assert_tokens {
            ($m:expr, $t1:expr, $t2:expr $(,)?) => {
                assert_eq!(
                    token::transform($m),
                    Some(vec![$t1("foo"), $t2("bar")]),
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
}
