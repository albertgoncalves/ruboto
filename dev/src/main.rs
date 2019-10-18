mod receive;
mod respond;

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

fn ping(out: sync::Arc<ws::Sender>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let receive: u8 = RECEIVE.load(sync::atomic::Ordering::SeqCst);
        if SEND.load(sync::atomic::Ordering::SeqCst) == receive {
            if receive == 0 {
                SEND.store(1, sync::atomic::Ordering::SeqCst);
                out.send(PING_1).unwrap();
            } else {
                SEND.store(0, sync::atomic::Ordering::SeqCst);
                out.send(PING_0).unwrap();
            }
        } else {
            exit(1)
        }
    });
}

fn interact(message: &str, bot_id: &str, _out: &ws::Sender) {
    println!("{}received{} {:?}", BOLD_BLUE, END, message);
    receive::token::transform(&message.replace("\\n", "\n"))
        .as_ref()
        .and_then(|tokens| receive::parse::transform(tokens))
        .map_or((), |payload| {
            println!("{}parsed{}   {:?}", BOLD_CYAN, END, payload);
            match payload {
                receive::parse::Parse::Pong("0") => {
                    RECEIVE.store(0, sync::atomic::Ordering::SeqCst)
                }
                receive::parse::Parse::Pong("1") => {
                    RECEIVE.store(1, sync::atomic::Ordering::SeqCst)
                }
                receive::parse::Parse::Message(m) => {
                    if m.user != bot_id {
                        let tokens: Option<Vec<respond::token::Token>> =
                            respond::token::transform(m.text);
                        println!(
                            "{}tokens{}   {:?}",
                            BOLD_YELLOW, END, tokens,
                        );
                        println!(
                            "{}response{} {:?}",
                            BOLD_PINK,
                            END,
                            tokens.and_then(|tokens| {
                                respond::parse::transform(&tokens)
                            }),
                        )
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
