use std::{
    os::unix::process::{CommandExt, ExitStatusExt},
    process::ExitStatus,
};

/// Exit with an exit code
pub fn exec(args: &[&str]) -> ExitStatus {
    if args.is_empty() {
        return ExitStatusExt::from_raw(1);
    }

    let err = std::process::Command::new(args[0]).args(&args[1..]).exec();
    eprintln!("Exec error: {}", err);
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_err() {
        assert_eq!(exec(&[]), ExitStatusExt::from_raw(1));
    }
}
