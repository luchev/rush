use conch_parser::{
    ast::TopLevelCommand,
    lexer::Lexer,
    parse::{DefaultParser, ParseError},
};
use rustyline::{error::ReadlineError};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum PromptResult {
    Commands(Vec<TopLevelCommand<String>>),
    Eof,
    Interrupt,
    Error(ParseError<String>),
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
    pub fn next(&self, rl: &mut rustyline::Editor<()>) -> PromptResult {
        let mut line = String::new();

        loop {
            match rl.readline(self.ps1.as_ref()) {
                Ok(input) => {
                    line.push_str(input.as_ref());
                    if line.len() >= 2 && &line[line.len() - 1..] == "\\" {
                        line = String::from(&line[..line.len() - 1]);
                        PromptResult::Error(ParseError::Custom("".to_string()))
                    } else {
                        let lexer = Lexer::new(line.chars());
                        match DefaultParser::new(lexer)
                            .into_iter()
                            .collect::<std::result::Result<Vec<TopLevelCommand<String>>, _>>()
                        {
                            Ok(x) => {
                                rl.add_history_entry(line.clone());
                                return PromptResult::Commands(x)
                            },
                            Err(x) => PromptResult::Error(ParseError::Custom(x.to_string())),
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    return PromptResult::Interrupt;
                }
                Err(ReadlineError::Eof) => {
                    return PromptResult::Eof;
                }
                Err(err) => {
                    println!(": {:?}", err);
                    return PromptResult::Interrupt;
                }
            };
        }
    }
}
