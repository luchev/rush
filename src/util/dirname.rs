use std::{os::unix::process::ExitStatusExt, path::Path, process::ExitStatus};

/// Return directory portion of pathname
pub fn dirname(args: &[&str]) -> ExitStatus {
    for arg in args {
        match dirname_one(arg) {
            Ok(x) => println!("{}", x),
            Err(x) => {
                eprintln!("{}", x);
                return ExitStatusExt::from_raw(1);
            }
        }
    }
    ExitStatusExt::from_raw(0)
}

fn dirname_one(path: &str) -> Result<String, String> {
    if !path.contains('/') {
        Ok(String::from("."))
    } else {
        match Path::new(path)
            .parent()
            .map(|x| x.as_os_str())
            .map(|x| x.to_str())
            .map(|x| x.unwrap())
        {
            Some(x) => Ok(x.to_string()),
            None => Err(format!("{} has no parent directory\n", path)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirname_one() {
        assert_eq!("/", dirname_one("/tmp").unwrap().trim());
        assert_eq!("/usr", dirname_one("/usr/bin").unwrap().trim());
        assert_eq!("/usr", dirname_one("/usr/bin/").unwrap().trim());
        assert_eq!("/tmp", dirname_one("/tmp/file.txt").unwrap().trim());
        assert_eq!(".", dirname_one("file.txt").unwrap().trim());
        assert_eq!(".", dirname_one("./file.txt").unwrap().trim());
        assert_eq!(".", dirname_one(".").unwrap().trim());

        let err = dirname_one("/");
        assert!(err.is_err());
    }

    #[test]
    fn test_basename() {
        assert!(dirname(&["/tmp"]).success());
        assert!(dirname(&["/tmp", "/usr/bin"]).success());

        assert!(!dirname(&["/", "/usr/bin"]).success());
        assert!(!dirname(&["/usr/bin", "/"]).success());
    }
}
