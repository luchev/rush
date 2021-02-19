use crate::util;
use conch_parser::ast::*;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    os::unix::process::ExitStatusExt,
    process::{Child, ExitStatus},
    rc::Rc,
    sync::{Arc, Mutex},
};

lazy_static! {
    static ref UTIL_COMMANDS: HashMap::<&'static str, fn(&[&str]) -> ExitStatus> = {
        let mut map = HashMap::<&'static str, fn(&[&str]) -> ExitStatus>::new();
        map.insert("cd", util::cd::cd);
        map.insert("basename", util::basename::basename);
        map.insert("dirname", util::dirname::dirname);
        map.insert("pwd", util::pwd::pwd);
        map.insert("exit", util::exit::exit);
        map.insert("exec", util::exec::exec);
        map
    };
    pub static ref CURRENT_CHILD: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
}

type PipeCommand = PipeableCommand<
    String,
    Box<SimpleCommand<String, TopLevelWord<String>, Redirect<TopLevelWord<String>>>>,
    Box<
        CompoundCommand<
            CompoundCommandKind<String, TopLevelWord<String>, TopLevelCommand<String>>,
            Redirect<TopLevelWord<String>>,
        >,
    >,
    Rc<
        CompoundCommand<
            CompoundCommandKind<String, TopLevelWord<String>, TopLevelCommand<String>>,
            Redirect<TopLevelWord<String>>,
        >,
    >,
>;

type ListCommand = AndOrList<
    ListableCommand<
        PipeableCommand<
            String,
            Box<SimpleCommand<String, TopLevelWord<String>, Redirect<TopLevelWord<String>>>>,
            Box<
                CompoundCommand<
                    CompoundCommandKind<String, TopLevelWord<String>, TopLevelCommand<String>>,
                    Redirect<TopLevelWord<String>>,
                >,
            >,
            Rc<
                CompoundCommand<
                    CompoundCommandKind<String, TopLevelWord<String>, TopLevelCommand<String>>,
                    Redirect<TopLevelWord<String>>,
                >,
            >,
        >,
    >,
>;

type SingleCommand = PipeableCommand<
    String,
    Box<SimpleCommand<String, TopLevelWord<String>, Redirect<TopLevelWord<String>>>>,
    Box<
        CompoundCommand<
            CompoundCommandKind<String, TopLevelWord<String>, TopLevelCommand<String>>,
            Redirect<TopLevelWord<String>>,
        >,
    >,
    Rc<
        CompoundCommand<
            CompoundCommandKind<String, TopLevelWord<String>, TopLevelCommand<String>>,
            Redirect<TopLevelWord<String>>,
        >,
    >,
>;

struct Executable<'a> {
    command: &'a str,
    args: &'a [&'a str],
}

impl<'a> Executable<'a> {
    fn from_vec(command: &'a [&'a str]) -> Executable<'a> {
        Executable {
            command: command[0],
            args: &command[1..],
        }
    }
}

pub fn execute(commands: Vec<TopLevelCommand<String>>) -> Result<ExitStatus, &'static str> {
    commands
        .into_iter()
        .map(execute_toplevel_command)
        .last()
        .unwrap_or(Err(""))
}

fn execute_toplevel_command(command: TopLevelCommand<String>) -> Result<ExitStatus, &'static str> {
    match command {
        TopLevelCommand(Command::List(x)) => execute_list(x),
        TopLevelCommand(Command::Job(x)) => execute_list(x),
    }
}

fn execute_list(command: ListCommand) -> Result<ExitStatus, &'static str> {
    let AndOrList { first, rest } = command;
    let mut status = execute_listable(first);

    if rest.is_empty() {
        return status;
    }

    for command in rest {
        match (command, status) {
            (AndOr::And(command), Ok(exit_status)) => {
                if exit_status.success() {
                    status = execute_listable(command);
                } else {
                    break;
                }
            }
            (AndOr::Or(command), Ok(exit_status)) => {
                if !exit_status.success() {
                    status = execute_listable(command);
                } else {
                    break;
                }
            }
            (AndOr::Or(command), Err(_)) => {
                status = execute_listable(command);
            }
            _ => break,
        };
    }

    status
}

fn execute_listable(command: ListableCommand<PipeCommand>) -> Result<ExitStatus, &'static str> {
    match command {
        ListableCommand::Pipe(negate_last, command) => {
            let status = execute_pipe(command)?;
            if negate_last {
                if status.success() {
                    Ok(ExitStatusExt::from_raw(1))
                } else {
                    Ok(ExitStatusExt::from_raw(0))
                }
            } else {
                Ok(status)
            }
        }
        ListableCommand::Single(command) => execute_single(command),
    }
}

fn execute_pipe(_command: Vec<PipeCommand>) -> Result<ExitStatus, &'static str> {
    Err("Unsupported: Pipe")
}

fn execute_single(command: SingleCommand) -> Result<ExitStatus, &'static str> {
    match command {
        PipeableCommand::Simple(command) => execute_simple(command),
        PipeableCommand::Compound(command) => {
            if !command.io.is_empty() {
                return Err("Unimplemented: Compound> command io");
            }

            match command.kind {
                CompoundCommandKind::Subshell(commands) => execute_subshell(commands),
                CompoundCommandKind::Brace(_commands) => Err("Unsupported: Compound command Brace"),
                CompoundCommandKind::While(_guard_body_pair) => {
                    Err("Unsupported: Compound command While")
                }
                CompoundCommandKind::Until(_guard_body_pair) => {
                    Err("Unsupported: Compound command Until")
                }
                CompoundCommandKind::If {
                    conditionals: _,
                    else_branch: _,
                } => Err("Unsupported: Compound command If"),
                CompoundCommandKind::For {
                    var: _,
                    words: _,
                    body: _,
                } => Err("Unsupported: Compound command For"),
                CompoundCommandKind::Case { word: _, arms: _ } => {
                    Err("Unsupported: Compound command Case")
                }
            }
        }
        PipeableCommand::FunctionDef(_name, _body) => Err("Unsupported: Function definition"),
    }
}

fn execute_subshell(commands: Vec<TopLevelCommand<String>>) -> Result<ExitStatus, &'static str> {
    use nix::{
        sys::wait::{wait,WaitStatus},
        unistd::{
            fork,
            ForkResult::{Child, Parent},
        },
    };

    unsafe {
        let pid = fork();
        match pid.expect("Fork Failed: Unable to create child process!") {
            Child => {
                match execute(commands) {
                    Ok(status) => std::process::exit(status.code().unwrap_or(1)),
                    Err(x) => {
                        eprintln!("Error in subshell: {}", x);
                        std::process::exit(1);
                    }
                }
            }
            Parent { child: _ } => {
                match wait() {
                    Ok(WaitStatus::Exited(_, status)) => {
                        Ok(ExitStatusExt::from_raw(status))
                    },
                    err => {
                        eprintln!("Error with subshell execution: {:?}", err);
                        Err("Failed to execute subshell")
                    }
                }
            }
        }
    }
}

fn execute_simple(
    command: Box<SimpleCommand<String, TopLevelWord<String>, Redirect<TopLevelWord<String>>>>,
) -> Result<ExitStatus, &'static str> {
    let SimpleCommand {
        redirects_or_env_vars,
        redirects_or_cmd_words,
    } = command.as_ref();
    if !redirects_or_env_vars.is_empty() {
        return Err("Unsupported: environment variables or redirects");
    }
    let mut args = vec![];
    for word in redirects_or_cmd_words {
        match word {
            RedirectOrCmdWord::CmdWord(TopLevelWord(word)) => match word {
                ComplexWord::Single(word) => match word {
                    Word::Simple(word) => match word {
                        SimpleWord::Literal(x) => args.push(x.as_ref()),
                        SimpleWord::Escaped(x) => args.push(x.as_ref()),
                        SimpleWord::Colon => {
                            return Err("Unsupported: :");
                        }
                        SimpleWord::Param(_x) => {
                            return Err("Unsupported: Params");
                        }
                        SimpleWord::Question => {
                            return Err("Unsupported: ?");
                        }
                        SimpleWord::SquareClose => {
                            return Err("Unsupported: [");
                        }
                        SimpleWord::SquareOpen => {
                            return Err("Unsupported: ]");
                        }
                        SimpleWord::Star => {
                            return Err("Unsupported: *");
                        }
                        SimpleWord::Subst(_x) => {
                            return Err("Unsupported: *");
                        }
                        SimpleWord::Tilde => {
                            return Err("Unsupported: ~");
                        }
                    },
                    Word::SingleQuoted(word) => args.push(word.as_ref()),
                    Word::DoubleQuoted(word) => {
                        println!("Concat: {:?}", word);
                    }
                },
                ComplexWord::Concat(_word) => {
                    return Err("Unsupported: Concat word");
                }
            },
            RedirectOrCmdWord::Redirect(redirect) => {
                println!("{:?}", redirect);
            }
        }
    }

    let executable = Executable::from_vec(&args);
    run(executable)
}

fn run(executable: Executable) -> Result<ExitStatus, &'static str> {
    if let Ok(execution_result) = run_internal(executable.command, executable.args) {
        Ok(execution_result)
    } else {
        use std::process::Command;

        match Command::new(executable.command)
            .args(executable.args)
            .spawn()
        {
            Ok(x) => {
                *CURRENT_CHILD.lock().unwrap() = Some(x);
                match CURRENT_CHILD.lock().unwrap().as_mut().unwrap().wait() {
                    Ok(x) => Ok(x),
                    Err(x) => {
                        eprintln!("{}", x);
                        Err("IO Error.")
                    }
                }
            }
            Err(x) => {
                eprintln!("{}", x);
                Err("Failed to spawn")
            }
        }
    }
}

fn run_internal<'a>(command: &'a str, args: &'a [&'a str]) -> Result<ExitStatus, &'static str> {
    match UTIL_COMMANDS.get(command) {
        Some(util_function) => Ok(util_function(args)),
        None => Err("Not an internal command"),
    }
}
