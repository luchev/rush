use std::{os::unix::process::ExitStatusExt, path::Path, process::ExitStatus};

/// Return non-directory portion of pathname
pub fn basename(args: &[&str]) -> ExitStatus {
    for arg in args {
        match basename_one(arg) {
            Ok(x) => println!("{}", x),
            Err(x) => {
                eprintln!("{}", x);
                return ExitStatusExt::from_raw(1);
            }
        }
    }
    ExitStatusExt::from_raw(0)
}

fn basename_one(path: &str) -> Result<String, String> {
    if !path.contains('/') {
        Ok(String::from(path))
    } else {
        match Path::new(path)
            .file_name()
            .map(|x| x.to_str())
            .map(|x| x.unwrap())
        {
            Some(x) => Ok(x.to_string()),
            None => Err(format!("{} has no basename\n", path)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basename_one() {
        assert_eq!("tmp", basename_one("/tmp").unwrap().trim());
        assert_eq!("bin", basename_one("/usr/bin").unwrap().trim());
        assert_eq!("bin", basename_one("/usr/bin/").unwrap().trim());
        assert_eq!("file.txt", basename_one("/tmp/file.txt").unwrap().trim());
        assert_eq!("file.txt", basename_one("file.txt").unwrap().trim());
        assert_eq!("file.txt", basename_one("./file.txt").unwrap().trim());
        assert_eq!(".", basename_one(".").unwrap().trim());

        let err = basename_one("/");
        assert!(err.is_err());
    }

    #[test]
    fn test_basename() {
        assert!(basename(&["/tmp"]).success());
        assert!(basename(&["/tmp", "/usr/bin"]).success());

        assert!(!basename(&["/", "/usr/bin"]).success());
        assert!(!basename(&["/usr/bin", "/"]).success());
    }
}
