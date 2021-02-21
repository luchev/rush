use std::{os::unix::process::ExitStatusExt, process::ExitStatus};

/// Exit with an exit code
pub fn exit(args: &[&str]) -> ExitStatus {
    if args.len() >= 2 {
        eprintln!("Too many arguments");
        return ExitStatusExt::from_raw(1);
    }

    if args.len() == 1 {
        match args[0].parse::<i32>() {
            Ok(exit_code) => std::process::exit(exit_code),
            Err(_) => {
                eprintln!("Expected argument to be an integer");
                return ExitStatusExt::from_raw(2);
            }
        }
    }

    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_err() {
        assert_eq!(exit(&["123", "123"]), ExitStatusExt::from_raw(1));
        assert_eq!(exit(&["wrong args"]), ExitStatusExt::from_raw(2));
    }
}
