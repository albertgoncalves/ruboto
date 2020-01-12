use crate::terminal;
use std::process;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use ws::Sender;

const EXIT_FAILURE: i32 = 1;
const WAIT: u64 = 10;
const PING_0: &str = r#"{"id": 0, "type": "ping"}"#;
const PING_1: &str = r#"{"id": 1, "type": "ping"}"#;

pub static SEND: AtomicU8 = AtomicU8::new(0);
pub static RECEIVE: AtomicU8 = AtomicU8::new(0);

#[macro_export]
macro_rules! store {
    ($a:expr, $v:expr $(,)?) => {
        $a.store($v, Ordering::SeqCst)
    };
}

#[macro_export]
macro_rules! load {
    ($a:expr $(,)?) => {
        $a.load(Ordering::SeqCst)
    };
}

macro_rules! print_ping {
    ($p:expr $(,)?) => {
        println!("{}ping{}     {:?}", terminal::BOLD_WHITE, terminal::END, $p)
    };
}

pub fn ping(out: Arc<Sender>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(WAIT));
        let receive: u8 = load!(RECEIVE);
        if load!(SEND) == receive {
            if receive == 0 {
                store!(SEND, 1);
                out.send(PING_1).unwrap();
                print_ping!(1);
            } else {
                store!(SEND, 0);
                out.send(PING_0).unwrap();
                print_ping!(0);
            }
        } else {
            process::exit(EXIT_FAILURE)
        }
    });
}
