mod config;
mod executer;
mod libc_bindings;
mod prompt;
mod signals;
mod util;
mod globals;
use crate::{config::Config, executer::execute};
use std::process;

fn main() {
    signals::init();

    let mut conf = Config::default();
    conf.load();
    loop {
        match conf.prompt.next() {
            prompt::PromptResult::Commands(x) => {
                let status = execute(x);
                if let Err(x) = status {
                    eprintln!("Prompt error: {}", x)
                }
            }
            prompt::PromptResult::Error(x) => println!("{}", x),
            prompt::PromptResult::Eof => process::exit(0),
        }
    }
}
