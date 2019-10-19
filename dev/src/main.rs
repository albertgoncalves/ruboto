mod receive;
mod respond;
mod test;

use std::borrow::Cow;
use std::env::var;
use std::process::exit;
use std::sync;
use std::thread;
use std::time::Duration;
use ws;

const BOLD_BLUE: &str = "\x1b[1;34m";
const BOLD_CYAN: &str = "\x1b[1;36m";
const BOLD_PINK: &str = "\x1b[1;35m";
const BOLD_YELLOW: &str = "\x1b[1;33m";
const END: &str = "\x1b[0m";
const PING_0: &str = r#"{"id": 0, "type": "ping"}"#;
const PING_1: &str = r#"{"id": 1, "type": "ping"}"#;

static SEND: sync::atomic::AtomicU8 = sync::atomic::AtomicU8::new(0);
static RECEIVE: sync::atomic::AtomicU8 = sync::atomic::AtomicU8::new(0);

macro_rules! store {
    ($a:expr, $v:expr $(,)?) => {
        $a.store($v, sync::atomic::Ordering::SeqCst)
    };
}

macro_rules! load {
    ($a:expr $(,)?) => {
        $a.load(sync::atomic::Ordering::SeqCst)
    };
}

fn ping(out: sync::Arc<ws::Sender>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let receive: u8 = load!(RECEIVE);
        if load!(SEND) == receive {
            if receive == 0 {
                store!(SEND, 1);
                out.send(PING_1).unwrap();
            } else {
                store!(SEND, 0);
                out.send(PING_0).unwrap();
            }
        } else {
            exit(1)
        }
    });
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

fn sanitize(input: &str) -> String {
    let n: usize = input.len();
    let mut output: String = String::with_capacity(n);
    let chars: Vec<char> = input.chars().collect::<Vec<char>>();
    match chars[0] {
        '\\' => (),
        '\n' => output.push(' '),
        c => output.push(c),
    }
    for i in 0..(n - 1) {
        match (chars[i], chars[i + 1]) {
            ('\\', 'n') | (_, '\n') => output.push(' '),
            (_, '\\') => (),
            (_, c) => output.push(c),
        }
    }
    output
}

fn interact(message: &str, bot_id: &str, out: &ws::Sender) {
    println!("{}received{} {:?}", BOLD_BLUE, END, message);
    receive::token::transform(message)
        .as_ref()
        .and_then(|tokens| receive::parse::transform(tokens))
        .map_or((), |payload| {
            println!("{}parsed{}   {:?}", BOLD_CYAN, END, payload);
            match payload {
                receive::parse::Parse::Pong("0") => store!(RECEIVE, 0),
                receive::parse::Parse::Pong("1") => store!(RECEIVE, 1),
                receive::parse::Parse::Message(m) => {
                    if m.user != bot_id {
                        let text: String = sanitize(m.text);
                        let tokens: Option<Vec<respond::token::Token>> =
                            respond::token::transform(&text);
                        println!(
                            "{}tokens{}   {:?}",
                            BOLD_YELLOW, //
                            END,         //
                            tokens,
                        );
                        let response: Option<Cow<str>> =
                            tokens.and_then(|tokens| {
                                respond::parse::transform(&tokens)
                            });
                        println!(
                            "{}response{} {:?}",
                            BOLD_PINK, //
                            END,       //
                            response
                        );
                        if let Some(r) = response {
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
        ping(out.clone());
        move |message: ws::Message| {
            message
                .into_text()
                .map(|message: String| interact(&message, &bot_id, &out))
        }
    })
    .unwrap()
}
