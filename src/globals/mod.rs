use crate::util;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    process::{Child, ExitStatus},
    sync::{Arc, Mutex},
};

pub const CONF_FILE_NAME: &str = ".rush";

lazy_static! {
    pub static ref CURRENT_CHILD: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    pub static ref UTIL_COMMANDS: HashMap::<&'static str, fn(&[&str]) -> ExitStatus> = {
        let mut map = HashMap::<&'static str, fn(&[&str]) -> ExitStatus>::new();
        map.insert("cd", util::cd::cd);
        map.insert("basename", util::basename::basename);
        map.insert("dirname", util::dirname::dirname);
        map.insert("pwd", util::pwd::pwd);
        map.insert("exit", util::exit::exit);
        map.insert("exec", util::exec::exec);
        map
    };
}

pub const STDIN: u16 = 0;
pub const STDOUT: u16 = 1;
// pub const STDERR: u16 = 2;
