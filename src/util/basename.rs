use std::path::Path;

/// Return non-directory portion of pathname
pub fn basename(args: &[&str]) -> Vec<Result<String, String>> {
    args.iter().map(|x| basename_one(x)).collect()
}

fn basename_one(path: &str) -> Result<String, String> {
    if !path.contains("/") {
        Ok(String::from(path))
    } else {
        match Path::new(path).file_name().map(|x| x.to_str()).map(|x| x.unwrap()) {
            Some(x) => Ok(format!("{}{}", x, '\n')),
            None => Err(format!("{} has no basename\n", path).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use super::*;

    #[test]
    fn test_basename() {
        assert_eq!("tmp", basename(&["/tmp"])[0].as_ref().unwrap().trim());
        assert_eq!("bin", basename(&["/usr/bin"])[0].as_ref().unwrap().trim());
        assert_eq!("bin", basename(&["/usr/bin/"])[0].as_ref().unwrap().trim());
        assert_eq!("file.txt", basename(&["/tmp/file.txt"])[0].as_ref().unwrap().trim());
        assert_eq!("file.txt", basename(&["file.txt"])[0].as_ref().unwrap().trim());
        assert_eq!("file.txt", basename(&["./file.txt"])[0].as_ref().unwrap().trim());
        assert_eq!(".", basename(&["."])[0].as_ref().unwrap().trim());

        let err = basename(&["/"]);
        assert!(err.len() == 1);
        assert!(err[0].is_err());
    }

}
