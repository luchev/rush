use std::{env, os::unix::process::ExitStatusExt, process::ExitStatus};

/// Return directory portion of pathname
pub fn pwd(args: &[&str]) -> ExitStatus {
    if !args.is_empty() {
        eprintln!("Too many arguments");
        ExitStatusExt::from_raw(1)
    } else {
        match env::current_dir() {
            Ok(x) => {
                println!("{}", x.into_os_string().to_str().unwrap());
                ExitStatusExt::from_raw(0)
            }
            Err(x) => {
                eprintln!("{}", x);
                ExitStatusExt::from_raw(2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pwd() {
        let _ = env::set_current_dir("/");
        assert!(pwd(&[]).success());

        let _ = env::set_current_dir("/usr");
        assert!(pwd(&[]).success());

        assert!(!pwd(&["home"]).success());
    }
}
