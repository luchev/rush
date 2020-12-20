use std::path::Path;
use std::ffi::OsString;

/// Return directory portion of pathname
fn dirname(paths: &[&str]) -> Vec<Result<OsString, OsString>> {
    paths.iter().map(|x| dirname_one(x)).collect()
}

fn dirname_one(path: &str) -> Result<OsString, OsString> {
    if !path.contains("/") {
        Ok(OsString::from("."))
    } else {
        match Path::new(path).parent().map(|x| x.as_os_str()) {
            Some(x) => Ok(OsString::from(x)),
            None => Err(format!("{} has no parent directory", path).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use super::*;

    #[test]
    fn test_dirname() {
        assert_eq!("/", dirname(&["/tmp"])[0].as_ref().unwrap());
        assert_eq!("/usr", dirname(&["/usr/bin"])[0].as_ref().unwrap());
        assert_eq!("/usr", dirname(&["/usr/bin/"])[0].as_ref().unwrap());
        assert_eq!("/tmp", dirname(&["/tmp/file.txt"])[0].as_ref().unwrap());
        assert_eq!(".", dirname(&["file.txt"])[0].as_ref().unwrap());
        assert_eq!(".", dirname(&["./file.txt"])[0].as_ref().unwrap());
        assert_eq!(".", dirname(&["."])[0].as_ref().unwrap());

        let err = dirname(&["/"]);
        assert!(err.len() == 1);
        assert!(err[0].is_err());
    }
}
