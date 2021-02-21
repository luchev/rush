mod config;
mod executer;
mod libc_bindings;
mod prompt;
mod signals;
mod util;
mod globals;
use crate::{config::Config, executer::{execute, ExecuteError}};
use std::process;
use rustyline::{Editor};

fn main() {
    signals::init();

    let mut conf = Config::default();
    conf.load();

    let mut rl = Editor::<()>::new();

    loop {
        match conf.prompt.next(&mut rl) {
            prompt::PromptResult::Commands(x) => {
                let status = execute(x);
                if let Err(x) = status {
                    if let ExecuteError::Empty = x {
                       // pass
                    } else {
                        eprintln!("Prompt error: {:?}", x)
                    }
                }
            }
            prompt::PromptResult::Error(x) => eprintln!("{}", x),
            prompt::PromptResult::Eof => process::exit(0),
            prompt::PromptResult::Interrupt => (),
        }
    }
}
