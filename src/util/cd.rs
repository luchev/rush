// cd ~username will put you in username's home directory.
use crate::libc_bindings::user_home_dir_by_user_name;
use std::{env, os::unix::process::ExitStatusExt, process::ExitStatus};

/// Return directory portion of pathname
pub fn cd(args: &[&str]) -> ExitStatus {
    if args.len() > 1 {
        eprintln!("Too many arguments");
        return ExitStatusExt::from_raw(1);
    }

    let next_dir;
    if args.is_empty() || args.len() == 1 && args[0] == "~" {
        next_dir = match env::var("HOME") {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Can't find HOME in ENV");
                return ExitStatusExt::from_raw(2);
            }
        }
    } else if args[0] == "-" {
        next_dir = match env::var("OLDPWD") {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Can't find OLDPWD in ENV");
                return ExitStatusExt::from_raw(3);
            }
        }
    } else if args[0].starts_with('~') {
        let home_dir = user_home_dir_by_user_name(&args[0][1..]);
        match home_dir {
            Ok(x) => next_dir = x,
            Err(_) => {
                eprintln!("Couldn't find home dir for user {}", &args[0][1..]);
                return ExitStatusExt::from_raw(4);
            }
        }
    } else {
        next_dir = args[0].into();
    }

    let old_pwd = env::current_dir().unwrap_or_default();
    match env::set_current_dir(next_dir) {
        Ok(_) => {
            env::set_var("OLDPWD", old_pwd);
            env::set_var("PWD", env::current_dir().unwrap_or_default());
            ExitStatusExt::from_raw(0)
        }
        Err(x) => {
            eprintln!("{}\n", x);
            ExitStatusExt::from_raw(5)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn test_cd_home() {
        let _ = env::set_current_dir("/");
        assert_eq!("/", env::current_dir().unwrap().into_os_string());

        // cd
        let _ = env::set_var("HOME", "/etc");
        let _ = cd(&[]);
        assert_eq!("/etc", env::current_dir().unwrap().into_os_string());

        // cd ~
        let _ = env::set_var("HOME", "/");
        let _ = cd(&["~"]);
        assert_eq!("/", env::current_dir().unwrap().into_os_string());

        // cd ~user
        if let Ok(user) = env::var("USER") {
            let _ = cd(&["/etc"]);
            let _ = cd(&[&format!("~{}", user)]);
            let home = user_home_dir_by_user_name(&user).unwrap_or_default();
            assert_eq!(
                OsString::from(home),
                env::current_dir().unwrap().into_os_string()
            );
        }
    }

    #[test]
    fn test_cd_parent_dir() {
        let _ = env::set_current_dir("/");
        assert_eq!("/", env::current_dir().unwrap().into_os_string());

        // cd ..
        let _ = env::set_current_dir("/etc");
        let _ = cd(&[".."]);
        assert_eq!("/", env::current_dir().unwrap().into_os_string());

        // cd ../..
        let _ = env::set_current_dir("/usr/bin");
        let _ = cd(&["../.."]);
        assert_eq!("/", env::current_dir().unwrap().into_os_string());
    }

    #[test]
    fn test_cd_previous_dir() {
        let _ = env::set_current_dir("/");
        assert_eq!("/", env::current_dir().unwrap().into_os_string());

        // cd -
        let _ = env::set_current_dir("/etc");
        let _ = cd(&[".."]);
        assert_eq!("/", env::current_dir().unwrap().into_os_string());
        let _ = cd(&["-"]);
        assert_eq!("/etc", env::current_dir().unwrap().into_os_string());
        let _ = cd(&["-"]);
        assert_eq!("/", env::current_dir().unwrap().into_os_string());
    }

    #[test]
    fn test_cd_absolute_path() {
        let _ = env::set_current_dir("/");
        assert_eq!("/", env::current_dir().unwrap().into_os_string());

        // cd /etc
        let _ = env::set_current_dir("/bin");
        let _ = cd(&["/etc"]);
        assert_eq!("/etc", env::current_dir().unwrap().into_os_string());

        // cd /
        let _ = env::set_current_dir("/bin");
        let _ = cd(&["/"]);
        assert_eq!("/", env::current_dir().unwrap().into_os_string());

        // cd etc
        let _ = env::set_current_dir("/");
        let _ = cd(&["etc"]);
        assert_eq!("/etc", env::current_dir().unwrap().into_os_string());
    }

    #[test]
    fn test_cd_relative_path() {
        let _ = env::set_current_dir("/");
        assert_eq!("/", env::current_dir().unwrap().into_os_string());

        // cd etc
        let _ = env::set_current_dir("/");
        let _ = cd(&["etc"]);
        assert_eq!("/etc", env::current_dir().unwrap().into_os_string());
    }

    #[test]
    fn test_cd() {
        assert!(cd(&["/"]).success());
        assert!(cd(&["/tmp"]).success());

        assert!(!cd(&[("/".to_string() + "😊".repeat(700).as_ref()).as_ref()]).success());
    }
}
