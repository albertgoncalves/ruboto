mod heartbeat;
mod receive;
mod respond;
mod terminal;
mod test;

use std::borrow::Cow;
use std::env::var;
use std::process::exit;
use std::sync;
use ws;

const BACKDOOR: &str = "!HALT";

macro_rules! backdoor {
    ($t:expr $(,)?) => {
        if $t == BACKDOOR {
            exit(0)
        }
    };
}

fn send(
    channel: &str,
    message: &str,
    out: &ws::Sender,
) -> Result<(), ws::Error> {
    let a: &str = "{\"id\":0,\"type\":\"message\",\"channel\":\"";
    let b: &str = "\",\"text\":\"";
    let c: &str = "\"}";
    let mut payload: String = String::with_capacity(
        a.len() + channel.len() + b.len() + message.len() + c.len(),
    );
    payload.push_str(a);
    payload.push_str(channel);
    payload.push_str(b);
    payload.push_str(message);
    payload.push_str(c);
    out.send(payload)
}

/* https://api.slack.com/docs/message-formatting */
fn sanitize(input: &str) -> String {
    let chars: Vec<char> = input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .chars()
        .collect::<Vec<char>>();
    let n: usize = chars.len();
    let mut output: String = String::with_capacity(n);
    match chars[0] {
        '\\' => (),
        '\n' => output.push(' '),
        c => output.push(c),
    }
    for i in 0..(n - 1) {
        match (chars[i], chars[i + 1]) {
            ('\\', 'n') | (_, '\n') => output.push(' '),
            (_, '\\')
            | (_, '*')
            | (_, '_')
            | (_, '~')
            | (_, '`')
            | (_, '>') => (),
            (_, c) => output.push(c),
        }
    }
    output
}

fn bot(text: &str) -> Option<Cow<'_, str>> {
    let tokens: Option<Vec<respond::token::Token>> =
        respond::token::transform(text);
    println!(
        "{}tokens{}   {:?}",
        terminal::BOLD_YELLOW,
        terminal::END,
        tokens,
    );
    let response: Option<Cow<str>> =
        tokens.and_then(|tokens| respond::parse::transform(&tokens));
    println!(
        "{}response{} {:?}",
        terminal::BOLD_PINK,
        terminal::END,
        response,
    );
    response
}

fn interact(message: &str, bot_id: &str, out: &ws::Sender) {
    println!(
        "{}received{} {:?}",
        terminal::BOLD_BLUE,
        terminal::END,
        message,
    );
    receive::token::transform(message)
        .as_ref()
        .and_then(|tokens| {
            println!(
                "{}tokens{}   {:?}",
                terminal::BOLD_PINK,
                terminal::END,
                tokens,
            );
            receive::parse::transform(tokens)
        })
        .map_or((), |payload| {
            println!(
                "{}parsed{}   {:?}",
                terminal::BOLD_CYAN,
                terminal::END,
                payload,
            );
            match payload {
                receive::parse::Parse::Pong("0") => {
                    store!(heartbeat::RECEIVE, 0)
                }
                receive::parse::Parse::Pong("1") => {
                    store!(heartbeat::RECEIVE, 1)
                }
                receive::parse::Parse::Message(m) => {
                    backdoor!(m.text);
                    if m.user != bot_id {
                        if let Some(r) = bot(&sanitize(m.text)) {
                            let _: Result<(), ws::Error> =
                                send(m.channel, &r, out);
                        }
                    }
                }
                _ => (),
            }
        });
    println!()
}

fn main() {
    ws::connect(var("URL").unwrap(), |out: ws::Sender| {
        let bot_id: String = var("BOT_ID").unwrap();
        let out: sync::Arc<ws::Sender> = sync::Arc::new(out);
        heartbeat::ping(out.clone());
        move |message: ws::Message| {
            message
                .into_text()
                .map(|message: String| interact(&message, &bot_id, &out))
        }
    })
    .unwrap();
    println!("{}end{}", terminal::BOLD_RED, terminal::END);
    exit(1);
}
