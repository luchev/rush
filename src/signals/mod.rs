use crate::globals::CURRENT_CHILD;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::thread;

pub fn init() {
    let mut signals = Signals::new(&[SIGINT]).unwrap();

    thread::spawn(move || {
        for sig in signals.forever() {
            if sig == SIGINT {
                let _ = CURRENT_CHILD.lock().unwrap().as_mut().map(|x| x.kill());
            }
        }
    });
}
