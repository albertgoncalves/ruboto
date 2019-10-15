use std::env;
use ws;

const BOLD_BLUE: &str = "\x1b[1;34m";
const END: &str = "\x1b[0m";
const PING_0: &str = r#"{"id": 0, "type": "ping"}"#;
const PING_1: &str = r#"{"id": 1, "type": "ping"}"#;

fn handle(message: &str, _out: &ws::Sender) {
    println!("{}received{} {}\n", BOLD_BLUE, END, message);
}

fn main() {
    ws::connect(env::var("URL").unwrap(), |out: ws::Sender| {
        let _: Result<(), ws::Error> = out.send(PING_0);
        let _: Result<(), ws::Error> = out.send(PING_1);
        move |message: ws::Message| {
            message
                .into_text()
                .map(|message: String| handle(&message, &out))
                .and(Ok(()))
        }
    })
    .unwrap()
}
