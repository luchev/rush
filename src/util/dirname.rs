use std::path::Path;

/// Return directory portion of pathname
pub fn dirname(paths: &[&str]) -> Vec<Result<String, String>> {
    paths.iter().map(|x| dirname_one(x)).collect()
}

fn dirname_one(path: &str) -> Result<String, String> {
    if !path.contains("/") {
        Ok(String::from("."))
    } else {
        match Path::new(path).parent().map(|x| x.as_os_str()).map(|x| x.to_str()).map(|x| x.unwrap()) {
            Some(x) => Ok(format!("{}{}", x, '\n')),
            None => Err(format!("{} has no parent directory\n", path).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use super::*;

    #[test]
    fn test_dirname() {
        assert_eq!("/", dirname(&["/tmp"])[0].as_ref().unwrap().trim());
        assert_eq!("/usr", dirname(&["/usr/bin"])[0].as_ref().unwrap().trim());
        assert_eq!("/usr", dirname(&["/usr/bin/"])[0].as_ref().unwrap().trim());
        assert_eq!("/tmp", dirname(&["/tmp/file.txt"])[0].as_ref().unwrap().trim());
        assert_eq!(".", dirname(&["file.txt"])[0].as_ref().unwrap().trim());
        assert_eq!(".", dirname(&["./file.txt"])[0].as_ref().unwrap().trim());
        assert_eq!(".", dirname(&["."])[0].as_ref().unwrap().trim());

        let err = dirname(&["/"]);
        assert!(err.len() == 1);
        assert!(err[0].is_err());
    }
}
