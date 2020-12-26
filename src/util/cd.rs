// cd ~username will put you in username's home directory.
use std::env;
use crate::libc_bindings::user_home_dir_by_user_name;

/// Return directory portion of pathname
pub fn cd(args: &[&str]) -> Vec<Result<String, String>> {
    if args.len() > 1 {
        return vec![Err("Too many arguments\n".into())];
    }

    let next_dir;
    if args.len() == 0 || args.len() == 1 && args[0] == "~" {
        next_dir = match env::var("HOME") {
            Ok(x) => x,
            Err(_) => return vec![Err("Can't find HOME in ENV\n".into())],
        }
    } else if args[0] == "-" {
        next_dir = match env::var("OLDPWD") {
            Ok(x) => x,
            Err(_) => return vec![Err("Can't find OLDPWD in ENV\n".into())],
        }
    } else if args[0].chars().next() == Some('~') {
        let home_dir = user_home_dir_by_user_name(&args[0][1..]);
        match home_dir {
            Ok(x) => next_dir = x,
            Err(_) => return vec![Err(format!("Couldn't find home dir for user {}", &args[0][1..]))],
        }
    } else {
        next_dir = args[0].into();
    }

    let old_pwd = env::current_dir().unwrap_or_default();
    match env::set_current_dir(next_dir) {
        Ok(_) => {
            env::set_var("OLDPWD", old_pwd);
            env::set_var("PWD", env::current_dir().unwrap_or_default());
            vec![]
        },
        Err(x) => vec![Err(format!("{}\n", x))],
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use std::ffi::OsString;
    use super::*;

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
            assert_eq!(OsString::from(home), env::current_dir().unwrap().into_os_string());
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
}
