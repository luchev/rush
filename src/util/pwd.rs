use std::env;

/// Return directory portion of pathname
pub fn pwd(args: &[&str]) -> Vec<Result<String, String>> {
    if args.len() > 0 {
        vec![Err(String::from("Too many arguments\n"))]
    } else {
        match env::current_dir() {
            Ok(x) => vec![Ok(format!("{}\n", x.into_os_string().to_str().unwrap()))],
            Err(x) => vec![Err(format!("{}\n", x))],
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use super::*;

    #[test]
    fn test_pwd() {
        let _ = env::set_current_dir("/");
        assert_eq!("/", pwd(&[])[0].as_ref().unwrap().trim());

        let _ = env::set_current_dir("/etc");
        assert_eq!("/etc", pwd(&[])[0].as_ref().unwrap().trim());

        let _ = env::set_current_dir("/usr");
        assert_eq!("/usr", pwd(&[])[0].as_ref().unwrap().trim());
        
        assert!(pwd(&["home"])[0].is_err());
    }
}
