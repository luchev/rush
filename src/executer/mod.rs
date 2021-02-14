use crate::util;
use conch_parser::ast::*;
use conch_parser::ast::{AndOrList, Command, ComplexWord, TopLevelCommand};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

lazy_static! {
    static ref UTIL_COMMANDS: HashMap::<&'static str, fn(&[&str]) -> ExitStatus> = {
        let mut map = HashMap::<&'static str, fn(&[&str]) -> ExitStatus>::new();
        map.insert("cd", util::cd::cd);
        map.insert("basename", util::basename::basename);
        map.insert("dirname", util::dirname::dirname);
        map.insert("pwd", util::pwd::pwd);
        map
    };
}

pub fn execute(commands: Vec<TopLevelCommand<String>>) -> Result<ExitStatus, String> {
    commands
        .into_iter()
        .map(|x| execute_toplevelcommand(x))
        .last()
        .unwrap()
}

fn execute_toplevelcommand(command: TopLevelCommand<String>) -> Result<ExitStatus, String> {
    println!("{:?}", command);
    if let TopLevelCommand(Command::List(AndOrList { first, rest })) = command {
        if let ListableCommand::Single(PipeableCommand::Simple(simple_command)) = first {
            let mut command = String::new();
            let mut args = vec![];
            for word in simple_command.redirects_or_cmd_words {
                match word {
                    RedirectOrCmdWord::CmdWord(TopLevelWord(ComplexWord::Single(x))) => match x {
                        Word::Simple(SimpleWord::Literal(x)) => args.push(x),
                        Word::DoubleQuoted(x) => println!("{:?}", x),
                        Word::SingleQuoted(x) => args.push(x),
                        _ => return Err("Unsupported literal encountered: ".to_string()),
                    },
                    _ => return Err("Unsupported Redirect".to_string()),
                }
                if command == "" {
                    command = args.pop().unwrap();
                }
            }
            match execute_single(command.as_ref(), &args.iter().map(|x| x as &str).collect::<Vec<&str>>()) {
                Ok(x) => println!("Exit code {}", x),
                Err(x) => eprintln!("{}", x),
            }
            Ok(ExitStatusExt::from_raw(0))
        } else {
            Err("Unsupported Listable command".to_string())
        }
    } else {
        Err("Unsupported TopLevel command".to_string())
    }
}

fn execute_single(command: &str, args: &[&str]) -> Result<ExitStatus, String> {
    if let Ok(execution_result) = execute_internal(command.as_ref(), args) {
        Ok(execution_result)
    } else {
        use std::process::Command;
        match Command::new(command).args(args).status() {
            Ok(x) => Ok(x),
            Err(x) => Err(x.to_string()),
        }
    }
}

fn execute_internal<'a>(command: &'a str, args: &[&str]) -> Result<ExitStatus, &'static str> {
    match UTIL_COMMANDS.get(command) {
        Some(util_function) => {
            Ok(util_function(args))
        }
        None => Err("Not an internal command"),
    }
}
