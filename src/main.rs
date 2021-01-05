mod libc_bindings;
mod prompt;
mod util;

use prompt::Prompt;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // simple_commands();

    let prompt = Prompt::new();
    while let command = prompt.next() {
        println!("{:?}", command);
    }
}

fn simple_commands() {
    let mut utils = HashMap::<&str, fn(&[&str]) -> Vec<Result<String, String>>>::new();
    utils.insert("cd", util::cd::cd);
    utils.insert("basename", util::basename::basename);
    utils.insert("dirname", util::dirname::dirname);
    utils.insert("pwd", util::pwd::pwd);

    loop {
        let _ = io::stdout().write_all(b"$ > ");
        let _ = io::stdout().flush();

        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        let command = buffer.split(" ").next().unwrap().trim();

        match utils.get(command) {
            Some(util_function) => {
                for result in util_function(
                    &buffer
                        .split(" ")
                        .into_iter()
                        .map(str::trim)
                        .collect::<Vec<&str>>()[1..],
                ) {
                    match result {
                        Ok(x) => {
                            let _ = io::stdout().write_all(x.as_bytes());
                        }
                        Err(x) => {
                            let _ = io::stderr().write_all(x.as_bytes());
                        }
                    }
                }
            }
            None => println!("{} is not a valid command.", command),
        }
    }
}
