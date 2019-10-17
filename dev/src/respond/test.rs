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
        assert_tokens!(
            "foo bar",
            token::Token::Content,
            token::Token::Content,
        );
        assert_tokens!(
            "foo\nbar",
            token::Token::Content,
            token::Token::Content,
        );
        assert_tokens!(
            "foo bar ",
            token::Token::Content,
            token::Token::Content,
        );
        assert_tokens!(
            " foo bar ",
            token::Token::Content,
            token::Token::Content,
        );
        assert_tokens!(
            "!foo bar",
            token::Token::Command,
            token::Token::Content,
        );
        assert_tokens!(
            "!foo\nbar",
            token::Token::Command,
            token::Token::Content,
        );
        assert_tokens!(
            "!foo bar ",
            token::Token::Command,
            token::Token::Content,
        );
        assert_tokens!(
            " !foo bar ",
            token::Token::Command,
            token::Token::Content,
        );
        assert_tokens!(
            "!foo !bar",
            token::Token::Command,
            token::Token::Command,
        );
        assert_tokens!(
            "!foo\n!bar",
            token::Token::Command,
            token::Token::Command,
        );
        assert_tokens!(
            "!foo !bar ",
            token::Token::Command,
            token::Token::Command,
        );
        assert_tokens!(
            " !foo !bar ",
            token::Token::Command,
            token::Token::Command,
        );
    }
}
