use std::env;
use std::ffi::OsString;

/// Return directory portion of pathname
fn pwd(paths: &[&str]) -> Vec<Result<OsString, OsString>> {
    if paths.len() > 0 {
        vec![Err(OsString::from("Too many arguments"))]
    } else {
        match env::current_dir() {
            Ok(x) => vec![Ok(x.into_os_string())],
            Err(x) => vec![Err(x.to_string().into())],
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
        assert_eq!("/", pwd(&[])[0].as_ref().unwrap());

        let _ = env::set_current_dir("/etc");
        assert_eq!("/etc", pwd(&[])[0].as_ref().unwrap());

        let _ = env::set_current_dir("/usr");
        assert_eq!("/usr", pwd(&[])[0].as_ref().unwrap());
        
        assert!(pwd(&["home"])[0].is_err());
    }
}
