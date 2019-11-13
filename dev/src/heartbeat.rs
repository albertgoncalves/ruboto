use crate::terminal;
use std::process::exit;
use std::sync;
use std::thread;
use std::time::Duration;

const PING_0: &str = r#"{"id": 0, "type": "ping"}"#;
const PING_1: &str = r#"{"id": 1, "type": "ping"}"#;

pub static SEND: sync::atomic::AtomicU8 = sync::atomic::AtomicU8::new(0);
pub static RECEIVE: sync::atomic::AtomicU8 = sync::atomic::AtomicU8::new(0);

#[macro_export]
macro_rules! store {
    ($a:expr, $v:expr $(,)?) => {
        $a.store($v, sync::atomic::Ordering::SeqCst)
    };
}

#[macro_export]
macro_rules! load {
    ($a:expr $(,)?) => {
        $a.load(sync::atomic::Ordering::SeqCst)
    };
}

macro_rules! print_ping {
    ($p:expr $(,)?) => {
        println!("{}ping{}     {:?}", terminal::BOLD_WHITE, terminal::END, $p)
    };
}

pub fn ping(out: sync::Arc<ws::Sender>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
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
            exit(1)
        }
    });
}
