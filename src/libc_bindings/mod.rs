use std::ffi::{CString, CStr};

pub fn user_home_dir_by_user_name(name: &str) -> Result<String, String> {
    let user = match CString::new(name) {
        Ok(x) => x,
        Err(_) => return Err("Failed to convert username to c-string".to_string()),
    };

    unsafe {
        let passwd_ptr = libc::getpwnam(user.as_ptr());
        let passwd = *passwd_ptr;
        let name = match CStr::from_ptr(passwd.pw_dir).to_str() {
            Ok(x) => x.to_string(),
            Err(_) => {
                return Err("Failed to convert user home directory to UTF8".to_string());
            }
        };

        Ok(name)
    }
}
