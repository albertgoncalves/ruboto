pub mod parse;
pub mod sanitize;
pub mod token;

#[cfg(test)]
mod test {
    use crate::receive::parse;
    use crate::receive::parse::Parse;
    use crate::receive::sanitize;
    use crate::receive::token;
    use crate::receive::token::Token;

    const MESSAGE: [Token; 83] = [
        Token::OpenBrace,
        Token::Quotation("client_msg_id"),
        Token::Colon,
        Token::Quotation("abcd-1234"),
        Token::Comma,
        Token::Quotation("suppress_notification"),
        Token::Colon,
        Token::Literal("false"),
        Token::Comma,
        Token::Quotation("type"),
        Token::Colon,
        Token::Quotation("message"),
        Token::Comma,
        Token::Quotation("text"),
        Token::Colon,
        Token::Quotation("\\\"hey\\\""),
        Token::Comma,
        Token::Quotation("user"),
        Token::Colon,
        Token::Quotation("USER1234"),
        Token::Comma,
        Token::Quotation("team"),
        Token::Colon,
        Token::Quotation("TEAM1234"),
        Token::Comma,
        Token::Quotation("blocks"),
        Token::Colon,
        Token::OpenBracket,
        Token::OpenBrace,
        Token::Quotation("type"),
        Token::Colon,
        Token::Quotation("rich_text"),
        Token::Comma,
        Token::Quotation("block_id"),
        Token::Colon,
        Token::Quotation("BLOCK1234"),
        Token::Comma,
        Token::Quotation("elements"),
        Token::Colon,
        Token::OpenBracket,
        Token::OpenBrace,
        Token::Quotation("type"),
        Token::Colon,
        Token::Quotation("rich_text_section"),
        Token::Comma,
        Token::Quotation("elements"),
        Token::Colon,
        Token::OpenBracket,
        Token::OpenBrace,
        Token::Quotation("type"),
        Token::Colon,
        Token::Quotation("text"),
        Token::Comma,
        Token::Quotation("text"),
        Token::Colon,
        Token::Quotation("\\\"hey\\\""),
        Token::CloseBrace,
        Token::CloseBracket,
        Token::CloseBrace,
        Token::CloseBracket,
        Token::CloseBrace,
        Token::CloseBracket,
        Token::Comma,
        Token::Quotation("user_team"),
        Token::Colon,
        Token::Quotation("USER_TEAM1234"),
        Token::Comma,
        Token::Quotation("source_team"),
        Token::Colon,
        Token::Quotation("SOURCE_TEAM1234"),
        Token::Comma,
        Token::Quotation("channel"),
        Token::Colon,
        Token::Quotation("CHANNEL1234"),
        Token::Comma,
        Token::Quotation("event_ts"),
        Token::Colon,
        Token::Quotation("1000000000.000000"),
        Token::Comma,
        Token::Quotation("ts"),
        Token::Colon,
        Token::Quotation("1000000000.000000"),
        Token::CloseBrace,
    ];

    const PONG: [Token; 9] = [
        Token::OpenBrace,
        Token::Quotation("type"),
        Token::Colon,
        Token::Quotation("pong"),
        Token::Comma,
        Token::Quotation("reply_to"),
        Token::Colon,
        Token::Literal("0"),
        Token::CloseBrace,
    ];

    fn compare_slices<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        let n: usize = a.len();
        if n != b.len() {
            return false;
        }
        for i in 0..n {
            if a[i] != b[i] {
                return false;
            }
        }
        true
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn sanitize() {
        macro_rules! assert_all {
            ($(($x:expr, $y:expr $(,)?),)*) => {
                $(assert_eq!(sanitize::sanitize($x), $y.to_owned());)*
            };
        }
        assert_all!(
            ("foo", "foo"),
            ("foo\\n", "foo "),
            ("foo", "foo"),
            ("foo\\n", "foo "),
            ("\\nfoo", " foo"),
            ("\\nfoo\\n", " foo "),
            ("\nfoo", " foo"),
            ("\\\"foo", "\"foo"),
            ("\\\"foo bar baz\\\"", "\"foo bar baz\""),
            ("\\\"foo\nbar\nbaz\\\"", "\"foo bar baz\""),
            ("\\\"foo\\nbar\\nbaz\\\"", "\"foo bar baz\""),
            ("\\\"foo\n \\nbar\n \\nbaz\\\"", "\"foo   bar   baz\""),
            ("foo &amp;", "foo &"),
            ("foo &lt;", "foo "),
            ("foo &gt;", "foo "),
            ("foo \\u201c", "foo \""),
            ("foo \\u201d", "foo \""),
            ("foo \\u2018", "foo \""),
            ("foo \\u2019", "foo \""),
            ("&amp; bar", "& bar"),
            ("&lt; bar", " bar"),
            ("&gt; bar", " bar"),
            ("\\u201c bar", "\" bar"),
            ("\\u201d bar", "\" bar"),
            ("\\u2018 bar", "\" bar"),
            ("\\u2019 bar", "\" bar"),
            (
                "&amp;&lt;&gt;\\u201c\\u201d\\u2018\\u2019\n\\*_~`",
                "&\"\"\"\" ",
            ),
        );
    }

    #[test]
    fn token_message() {
        if let Some(tokens) = token::transform(
            "{
                \"client_msg_id\":\"abcd-1234\",
                \"suppress_notification\":false,
                \"type\":\"message\",
                \"text\":\"\\\"hey\\\"\",
                \"user\":\"USER1234\",
                \"team\":\"TEAM1234\",
                \"blocks\":[
                    {
                        \"type\":\"rich_text\",
                        \"block_id\":\"BLOCK1234\",
                        \"elements\":[
                            {
                                \"type\":\"rich_text_section\",
                                \"elements\":[
                                    {
                                        \"type\":\"text\",
                                        \"text\":\"\\\"hey\\\"\"
                                    }
                                ]
                            }
                        ]
                    }
                ],
                \"user_team\":\"USER_TEAM1234\",
                \"source_team\":\"SOURCE_TEAM1234\",
                \"channel\":\"CHANNEL1234\",
                \"event_ts\":\"1000000000.000000\",
                \"ts\":\"1000000000.000000\"
            }",
        ) {
            assert!(compare_slices(&MESSAGE, &tokens))
        } else {
            panic!()
        }
    }

    #[test]
    fn parse_message() {
        assert_eq!(
            parse::transform(&MESSAGE),
            Some(Parse::Message(parse::Message {
                channel: "CHANNEL1234",
                text: "\\\"hey\\\"",
                user: "USER1234",
            }))
        )
    }

    #[test]
    fn token_pong() {
        if let Some(tokens) =
            token::transform(r#"{"type": "pong", "reply_to": 0}"#)
        {
            assert!(compare_slices(&PONG, &tokens))
        } else {
            panic!()
        }
    }

    #[test]
    fn token_and_parse_short_message() {
        let message: &str = "{
            \"channel\":\"CHANNEL1234\",
            \"client_msg_id\":\"abcd-1234\",
            \"event_ts\":\"1000000000.000000\",
            \"source_team\":\"SOURCE_TEAM1234\",
            \"suppress_notification\":false,
            \"team\":\"TEAM1234\",
            \"text\":\"\\\"hey\\\"\",
            \"ts\":\"1000000000.000000\",
            \"type\":\"message\",
            \"user\":\"USER1234\",
            \"user_team\":\"USER_TEAM1234\",
        }";
        assert_eq!(
            token::transform(message)
                .as_ref()
                .and_then(|xs| parse::transform(xs)),
            Some(Parse::Message(parse::Message {
                channel: "CHANNEL1234",
                text: "\\\"hey\\\"",
                user: "USER1234",
            }))
        )
    }

    #[test]
    fn parse_pong() {
        assert_eq!(parse::transform(&PONG), Some(Parse::Pong("0")))
    }

    #[test]
    fn token_nested() {
        if let Some(tokens) = token::transform(r#"{"foo": {"bar": "baz"}}"#) {
            compare_slices(
                &[
                    Token::OpenBrace,
                    Token::Quotation("foo"),
                    Token::Colon,
                    Token::OpenBrace,
                    Token::Quotation("bar"),
                    Token::Colon,
                    Token::Quotation("baz"),
                    Token::CloseBrace,
                    Token::CloseBrace,
                ],
                &tokens,
            );
        } else {
            panic!()
        }
    }

    #[test]
    fn token_empty() {
        assert_eq!(token::transform(""), None);
    }

    #[test]
    fn token_error() {
        assert_eq!(token::transform(r#"{\\"foo": "bar"}"#), None);
    }

    #[test]
    fn parse_empty() {
        assert_eq!(parse::transform(&[]), None);
        assert_eq!(
            parse::transform(&[Token::OpenBrace, Token::CloseBrace]),
            None,
        );
    }
}
