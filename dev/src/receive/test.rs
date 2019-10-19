#[cfg(test)]
mod test {
    use crate::receive::parse;
    use crate::receive::token;

    const MESSAGE: [token::Token; 45] = [
        token::Token::OpenBracket,
        token::Token::Quotation("client_msg_id"),
        token::Token::Colon,
        token::Token::Quotation("abcd-1234"),
        token::Token::Comma,
        token::Token::Quotation("suppress_notification"),
        token::Token::Colon,
        token::Token::Literal("false"),
        token::Token::Comma,
        token::Token::Quotation("type"),
        token::Token::Colon,
        token::Token::Quotation("message"),
        token::Token::Comma,
        token::Token::Quotation("text"),
        token::Token::Colon,
        token::Token::Quotation("\\\"hey\\\""),
        token::Token::Comma,
        token::Token::Quotation("user"),
        token::Token::Colon,
        token::Token::Quotation("USER1234"),
        token::Token::Comma,
        token::Token::Quotation("team"),
        token::Token::Colon,
        token::Token::Quotation("TEAM1234"),
        token::Token::Comma,
        token::Token::Quotation("user_team"),
        token::Token::Colon,
        token::Token::Quotation("USER_TEAM1234"),
        token::Token::Comma,
        token::Token::Quotation("source_team"),
        token::Token::Colon,
        token::Token::Quotation("SOURCE_TEAM1234"),
        token::Token::Comma,
        token::Token::Quotation("channel"),
        token::Token::Colon,
        token::Token::Quotation("CHANNEL1234"),
        token::Token::Comma,
        token::Token::Quotation("event_ts"),
        token::Token::Colon,
        token::Token::Quotation("1570906661.000200"),
        token::Token::Comma,
        token::Token::Quotation("ts"),
        token::Token::Colon,
        token::Token::Quotation("1570906661.000200"),
        token::Token::CloseBracket,
    ];
    const PONG: [token::Token; 9] = [
        token::Token::OpenBracket,
        token::Token::Quotation("type"),
        token::Token::Colon,
        token::Token::Quotation("pong"),
        token::Token::Comma,
        token::Token::Quotation("reply_to"),
        token::Token::Colon,
        token::Token::Literal("false"),
        token::Token::CloseBracket,
    ];

    fn compare_array_vector<T: PartialEq>(array: &[T], vector: &[T]) -> bool {
        for i in 0..array.len() {
            if array[i] != vector[i] {
                return false;
            }
        }
        true
    }

    #[test]
    fn token_message() {
        if let Some(tokens) = token::transform(
            r#"{
                "client_msg_id": "abcd-1234",
                "suppress_notification": false,
                "type": "message",
                "text": "\"hey\"",
                "user": "USER1234",
                "team": "TEAM1234",
                "user_team": "USER_TEAM1234",
                "source_team": "SOURCE_TEAM1234",
                "channel": "CHANNEL1234",
                "event_ts": "1570906661.000200",
                "ts": "1570906661.000200"
            }"#,
        ) {
            assert!(compare_array_vector(&MESSAGE, &tokens))
        } else {
            assert!(false)
        }
    }

    #[test]
    fn parse_message() {
        assert_eq!(
            parse::transform(&MESSAGE),
            Some(parse::Parse::Message(parse::Message {
                channel: "CHANNEL1234",
                text: "\\\"hey\\\"",
                user: "USER1234",
            }))
        )
    }

    #[test]
    fn token_pong() {
        if let Some(tokens) =
            token::transform(r#"{"type": "pong", "reply_to": false}"#)
        {
            assert!(compare_array_vector(&PONG, &tokens))
        } else {
            assert!(false)
        }
    }

    #[test]
    fn parse_pong() {
        assert_eq!(parse::transform(&PONG), Some(parse::Parse::Pong("false")))
    }

    #[test]
    fn token_nested() {
        if let Some(tokens) = token::transform(r#"{"foo": {"bar": "baz"}}"#) {
            assert_eq!(
                tokens,
                vec![
                    token::Token::OpenBracket,
                    token::Token::Quotation("foo"),
                    token::Token::Colon,
                    token::Token::OpenBracket,
                    token::Token::Quotation("bar"),
                    token::Token::Colon,
                    token::Token::Quotation("baz"),
                    token::Token::CloseBracket,
                    token::Token::CloseBracket,
                ],
            )
        } else {
            assert!(false)
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
        assert_eq!(parse::transform(&vec![]), None);
        assert_eq!(
            parse::transform(&vec![
                token::Token::OpenBracket,
                token::Token::CloseBracket,
            ]),
            None,
        );
    }
}
