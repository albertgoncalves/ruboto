#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate bencher;

mod receive;
mod respond;

use bencher::Bencher;
use receive::parse::Parse;
use receive::token::Token;
use std::borrow::Cow;

fn sanitize(b: &mut Bencher) {
    b.iter(|| {
        receive::sanitize::sanitize(
            "&amp;&lt;&gt;\\u201c\\u201d\\u2018\\u2019\n\\*_~`",
        )
    })
}

const MESSAGE: &str = "{
    \"client_msg_id\":\"abcd-1234\",
    \"suppress_notification\":false,
    \"type\":\"message\",
    \"text\":\"!JOIN \\\"foo bar\\\" baz\",
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
}";

fn receive_token(b: &mut Bencher) {
    b.iter(|| {
        let _: Option<Vec<Token>> = receive::token::transform(MESSAGE);
    })
}

fn receive_token_parse(b: &mut Bencher) {
    b.iter(|| {
        let _: Option<Parse> = receive::token::transform(MESSAGE)
            .as_ref()
            .and_then(|tokens| receive::parse::transform(tokens));
    })
}

fn receive_thru_respond(b: &mut Bencher) {
    let expected: Option<Cow<'_, str>> = Some(Cow::from("foo bar baz"));
    b.iter(|| {
        if let Some(Parse::Message(m)) = receive::token::transform(MESSAGE)
            .as_ref()
            .and_then(|tokens| receive::parse::transform(tokens))
        {
            let text: String = receive::sanitize::sanitize(m.text);
            let response: Option<Cow<'_, str>> =
                respond::token::transform(&text)
                    .as_ref()
                    .and_then(|tokens| respond::parse::transform(&tokens));
            assert_eq!(response, expected)
        } else {
            panic!()
        }
    })
}

benchmark_group!(
    benches,
    sanitize,
    receive_token,
    receive_token_parse,
    receive_thru_respond,
);
benchmark_main!(benches);
