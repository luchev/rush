use conch_parser::ast::TopLevelCommand;
use conch_parser::lexer::Lexer;
use conch_parser::parse::{DefaultParser, ParseError};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;

#[derive(Debug)]
pub enum PromptResult {
    Commands(Vec<TopLevelCommand<String>>),
    Eof,
    Error(ParseError::<String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Prompt {
    ps1: String,
    ps2: String,
    ps3: String,
    ps4: String,
    ps_quote: String,
    ps_dquote: String,
    ps_pipe: String,
    ps_and: String,
    ps_or: String,
}

impl Default for Prompt {
    fn default() -> Prompt {
        Prompt {
            ps1: String::from("$ "),
            ps2: String::from("$ "),
            ps3: String::from("$ "),
            ps4: String::from("$ "),
            ps_quote: String::from("' $ "),
            ps_dquote: String::from("\" $ "),
            ps_pipe: String::from("| $ "),
            ps_and: String::from("&& $ "),
            ps_or: String::from("|| $ "),
        }
    }
}

impl Prompt {
    pub fn next(&self) -> PromptResult {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut line = String::new();

        let _ = stdout.write(self.ps1.as_bytes());
        let _ = stdout.flush();
        let mut state = PromptResult::Error(ParseError::Custom("".to_string()));
        while let PromptResult::Error(_) = state {
            state = match stdin.read_line(&mut line) {
                Ok(0) => return PromptResult::Eof,
                Err(x) => PromptResult::Error(ParseError::Custom(x.to_string())),
                _ => {
                    if line.len() >= 2 && &line[line.len()-2..] == "\\\n" {
                        line = String::from(&line[..line.len()-2]);
                        PromptResult::Error(ParseError::Custom("".to_string()))
                    } else {
                        let lexer = Lexer::new(line.chars());
                        match DefaultParser::new(lexer).into_iter().collect::<std::result::Result<Vec<TopLevelCommand<String>>, _>>() {
                            Ok(x) => PromptResult::Commands(x),
                            Err(x) => PromptResult::Error(ParseError::Custom(x.to_string())),
                        }
                    }
                }
            };
        }

        state
    }
}
