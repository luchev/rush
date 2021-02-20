use crate::globals::{self, CURRENT_CHILD, UTIL_COMMANDS};
use conch_parser::ast::*;
use std::{fs::File, os::unix::process::ExitStatusExt, process::ExitStatus, rc::Rc};

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

#[derive(Debug)]
pub enum ExecuteError {
    StaticError(&'static str),
    Unsupported(&'static str),
    IoError(std::io::Error),
    NotAnInner,
    Empty,
}

struct Executable<'a> {
    command: &'a str,
    args: &'a [&'a str],
    redirects: Redirects,
}

impl<'a> Executable<'a> {
    fn from(command: &'a [&'a str], redirects: Redirects) -> Executable<'a> {
        Executable {
            command: command[0],
            args: &command[1..],
            redirects,
        }
    }
}

#[derive(Debug, Default)]
struct Redirects {
    read: Vec<(u16, String)>,
    write: Vec<(u16, String)>,
    read_write: Vec<(u16, String)>,
    append: Vec<(u16, String)>,
    clobber: Vec<(u16, String)>,
    dup_read: bool,
    dup_write: bool,
}

impl Redirects {
    fn parse(&mut self, redirect: &Redirect<TopLevelWord<String>>) {
        match redirect {
            Redirect::Read(from, to) => match parse_toplevel_word(to) {
                Ok(path) => self.write.push((from.unwrap_or(globals::STDIN), path)),
                Err(err) => eprintln!("Error parsing redirect: {:?}", err),
            },
            Redirect::Write(from, to) => match parse_toplevel_word(to) {
                Ok(path) => self.write.push((from.unwrap_or(globals::STDOUT), path)),
                Err(err) => eprintln!("Error parsing redirect: {:?}", err),
            },
            Redirect::ReadWrite(from, to) => match parse_toplevel_word(to) {
                Ok(path) => self.write.push((from.unwrap_or(globals::STDIN), path)),
                Err(err) => eprintln!("Error parsing redirect: {:?}", err),
            },
            Redirect::Append(from, to) => match parse_toplevel_word(to) {
                Ok(path) => self.write.push((from.unwrap_or(globals::STDOUT), path)),
                Err(err) => eprintln!("Error parsing redirect: {:?}", err),
            },
            Redirect::Clobber(_, _toplevel_word) => {
                eprintln!("Unsupported: Clobber");
            }
            Redirect::Heredoc(_, _toplevel_word) => {
                eprintln!("Unsupported: Heredoc");
            }
            Redirect::DupRead(from, to) => {
                eprintln!("{:?}, {:?}", from, to);
            }
            Redirect::DupWrite(from, to) => {
                eprintln!("{:?}, {:?}", from, to);
            }
        }
    }
}

fn parse_toplevel_word(word: &TopLevelWord<String>) -> Result<String, ExecuteError> {
    match word {
        TopLevelWord(word) => match word {
            ComplexWord::Single(word) => match word {
                Word::Simple(word) => match word {
                    SimpleWord::Literal(x) => Ok(x.to_string()),
                    SimpleWord::Escaped(x) => Ok(x.to_string()),
                    SimpleWord::Colon => Err(ExecuteError::Unsupported(":")),
                    SimpleWord::Param(_x) => Err(ExecuteError::Unsupported("Params")),
                    SimpleWord::Question => Err(ExecuteError::Unsupported("?")),
                    SimpleWord::SquareClose => Err(ExecuteError::Unsupported("[")),
                    SimpleWord::SquareOpen => Err(ExecuteError::Unsupported("]")),
                    SimpleWord::Star => Err(ExecuteError::Unsupported("*")),
                    SimpleWord::Subst(_x) => Err(ExecuteError::Unsupported("substring")),
                    SimpleWord::Tilde => Err(ExecuteError::Unsupported("~")),
                },
                Word::SingleQuoted(word) => Ok(word.to_string()),
                Word::DoubleQuoted(_word) => Err(ExecuteError::StaticError("DoubleQuoted")),
            },
            ComplexWord::Concat(_word) => Err(ExecuteError::StaticError("Concat word")),
        },
    }
}

pub fn execute(commands: Vec<TopLevelCommand<String>>) -> Result<ExitStatus, ExecuteError> {
    commands
        .into_iter()
        .map(execute_toplevel_command)
        .last().unwrap_or(Err(ExecuteError::Empty))
}

fn execute_toplevel_command(command: TopLevelCommand<String>) -> Result<ExitStatus, ExecuteError> {
    match command {
        TopLevelCommand(Command::List(x)) => execute_list(x),
        TopLevelCommand(Command::Job(x)) => execute_list(x),
    }
}

fn execute_list(command: ListCommand) -> Result<ExitStatus, ExecuteError> {
    let AndOrList { first, rest } = command;
    let mut status = execute_listable(first);

    if rest.is_empty() {
        return status;
    }

    for command in rest {
        match (command, &status) {
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

fn execute_listable(command: ListableCommand<PipeCommand>) -> Result<ExitStatus, ExecuteError> {
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

fn execute_pipe(commands: Vec<PipeCommand>) -> Result<ExitStatus, ExecuteError> {
    if commands.is_empty() {
        return Err(ExecuteError::StaticError("Invalid empty pipe command"));
    }
    if commands.len() == 1 {
        return execute_single(commands.into_iter().next().unwrap());
    }

    Err(ExecuteError::Unsupported("Pipe"))
}

fn execute_single(command: SingleCommand) -> Result<ExitStatus, ExecuteError> {
    match command {
        PipeableCommand::Simple(command) => execute_simple(command),
        PipeableCommand::Compound(command) => {
            if !command.io.is_empty() {
                return Err(ExecuteError::Unsupported("Compound> command io"));
            }

            match command.kind {
                CompoundCommandKind::Subshell(commands) => execute_subshell(commands),
                CompoundCommandKind::Brace(_commands) => Err(ExecuteError::Unsupported("Compound command Brace")),
                CompoundCommandKind::While(_guard_body_pair) => {
                    Err(ExecuteError::Unsupported("Compound command While"))
                }
                CompoundCommandKind::Until(_guard_body_pair) => {
                    Err(ExecuteError::Unsupported("Compound command Until"))
                }
                CompoundCommandKind::If {
                    conditionals: _,
                    else_branch: _,
                } => Err(ExecuteError::Unsupported("Compound command If")),
                CompoundCommandKind::For {
                    var: _,
                    words: _,
                    body: _,
                } => Err(ExecuteError::Unsupported("Compound command For")),
                CompoundCommandKind::Case { word: _, arms: _ } => {
                    Err(ExecuteError::Unsupported("Compound command Case"))
                }
            }
        }
        PipeableCommand::FunctionDef(_name, _body) => Err(ExecuteError::Unsupported("Function definition")),
    }
}

fn execute_subshell(commands: Vec<TopLevelCommand<String>>) -> Result<ExitStatus, ExecuteError> {
    use nix::{
        sys::wait::{wait, WaitStatus},
        unistd::{
            fork,
            ForkResult::{Child, Parent},
        },
    };

    unsafe {
        let pid = fork();
        match pid.expect("Fork Failed: Unable to create child process!") {
            Child => match execute(commands) {
                Ok(status) => std::process::exit(status.code().unwrap_or(1)),
                Err(x) => {
                    eprintln!("Error in subshell: {:?}", x);
                    std::process::exit(1);
                }
            },
            Parent { child: _ } => match wait() {
                Ok(WaitStatus::Exited(_, status)) => Ok(ExitStatusExt::from_raw(status)),
                err => {
                    eprintln!("Error with subshell execution: {:?}", err);
                    Err(ExecuteError::StaticError("Failed to execute subshell"))
                }
            },
        }
    }
}

fn execute_simple(
    command: Box<SimpleCommand<String, TopLevelWord<String>, Redirect<TopLevelWord<String>>>>,
) -> Result<ExitStatus, ExecuteError> {
    let SimpleCommand {
        redirects_or_env_vars,
        redirects_or_cmd_words,
    } = command.as_ref();
    if !redirects_or_env_vars.is_empty() {
        return Err(ExecuteError::StaticError("Unsupported: environment variables or redirects"));
    }

    let mut args = vec![];
    let mut redirects = Redirects::default();

    for word in redirects_or_cmd_words {
        match word {
            RedirectOrCmdWord::CmdWord(TopLevelWord(word)) => match word {
                ComplexWord::Single(word) => match word {
                    Word::Simple(word) => match word {
                        SimpleWord::Literal(x) => args.push(x.as_ref()),
                        SimpleWord::Escaped(x) => args.push(x.as_ref()),
                        SimpleWord::Colon => {
                            return Err(ExecuteError::Unsupported(":"));
                        }
                        SimpleWord::Param(_x) => {
                            return Err(ExecuteError::Unsupported("Params"));
                        }
                        SimpleWord::Question => {
                            return Err(ExecuteError::Unsupported("?"));
                        }
                        SimpleWord::SquareClose => {
                            return Err(ExecuteError::Unsupported("["));
                        }
                        SimpleWord::SquareOpen => {
                            return Err(ExecuteError::Unsupported("]"));
                        }
                        SimpleWord::Star => {
                            return Err(ExecuteError::Unsupported("*"));
                        }
                        SimpleWord::Subst(_x) => {
                            return Err(ExecuteError::Unsupported("substring"));
                        }
                        SimpleWord::Tilde => {
                            return Err(ExecuteError::Unsupported("~"));
                        }
                    },
                    Word::SingleQuoted(word) => args.push(word.as_ref()),
                    Word::DoubleQuoted(_word) => {
                        return Err(ExecuteError::Unsupported("DoubleQuoted"));
                    }
                },
                ComplexWord::Concat(_word) => {
                    return Err(ExecuteError::Unsupported("Concat word"));
                }
            },
            RedirectOrCmdWord::Redirect(redirect) => {
                redirects.parse(redirect);
            }
        }
    }

    let executable = Executable::from(&args, redirects);
    run(executable)
}

fn run(executable: Executable) -> Result<ExitStatus, ExecuteError> {
    if let Ok(execution_result) = run_internal(executable.command, executable.args) {
        Ok(execution_result)
    } else {
        use std::process::Command;

        let mut command = Command::new(executable.command);
        command.args(executable.args);

        // Parse write fd
        for (from, to) in executable.redirects.write {
            if from == 1 {
                match File::create(to) {
                    std::io::Result::Ok(file) => command.stdout(file),
                    std::io::Result::Err(_) => return Err(ExecuteError::StaticError("Failed to open file")),
                };
            } else if from == 2 {
                match File::create(to) {
                    std::io::Result::Ok(file) => command.stderr(file),
                    std::io::Result::Err(_) => return Err(ExecuteError::StaticError("Failed to open file")),
                };
            } else {
                todo!();
            }
        }

        // Parse read fd
        for (from, to) in executable.redirects.read {
            if from == 0 {
                match File::open(to) {
                    std::io::Result::Ok(file) => command.stdin(file),
                    std::io::Result::Err(_) => return Err(ExecuteError::StaticError("Failed to open file")),
                };
            } else {
                todo!();
            }
        }

        match command.spawn() {
            Ok(x) => {
                *CURRENT_CHILD.lock().unwrap() = Some(x);
                match CURRENT_CHILD.lock().unwrap().as_mut().unwrap().wait() {
                    Ok(x) => Ok(x),
                    Err(x) => {
                        Err(ExecuteError::IoError(x))
                    }
                }
            }
            Err(x) => {
                Err(ExecuteError::IoError(x))
            }
        }
    }
}

fn run_internal<'a>(command: &'a str, args: &'a [&'a str]) -> Result<ExitStatus, ExecuteError> {
    match UTIL_COMMANDS.get(command) {
        Some(util_function) => Ok(util_function(args)),
        None => Err(ExecuteError::NotAnInner),
    }
}
