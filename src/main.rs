mod config;
mod executer;
mod libc_bindings;
mod prompt;
mod util;
use crate::config::Config;
use crate::executer::execute;
use std::process;

fn main() {
    let mut conf = Config::default();
    conf.load();
    loop {
        match conf.prompt.next() {
            prompt::Result::Commands(x) => {
                let status = execute(x);
                match status {
                    Ok(x) => println!("{}", x),
                    Err(x) => eprintln!("{}", x),
                }
            }
            prompt::Result::Error(x) => println!("{}", x),
            prompt::Result::Eof => process::exit(0),
        }
    }
}
