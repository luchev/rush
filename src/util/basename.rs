use std::path::Path;
use std::ffi::OsString;

/// Return non-directory portion of pathname
fn basename(paths: &[&str]) -> Vec<Result<OsString, OsString>> {
    paths.iter().map(|x| basename_one(x)).collect()
}

fn basename_one(path: &str) -> Result<OsString, OsString> {
    if !path.contains("/") {
        Ok(OsString::from(path))
    } else {
        match Path::new(path).file_name() {
            Some(x) => Ok(OsString::from(x)),
            None => Err(format!("{} has no basename", path).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use super::*;

    #[test]
    fn test_basename() {
        assert_eq!("tmp", basename(&["/tmp"])[0].as_ref().unwrap());
        assert_eq!("bin", basename(&["/usr/bin"])[0].as_ref().unwrap());
        assert_eq!("bin", basename(&["/usr/bin/"])[0].as_ref().unwrap());
        assert_eq!("file.txt", basename(&["/tmp/file.txt"])[0].as_ref().unwrap());
        assert_eq!("file.txt", basename(&["file.txt"])[0].as_ref().unwrap());
        assert_eq!("file.txt", basename(&["./file.txt"])[0].as_ref().unwrap());
        assert_eq!(".", basename(&["."])[0].as_ref().unwrap());

        let err = basename(&["/"]);
        assert!(err.len() == 1);
        assert!(err[0].is_err());
    }

}
