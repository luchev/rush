use conch_parser::ast::TopLevelCommand;
use conch_parser::lexer::Lexer;
use conch_parser::parse::{DefaultParser, ParseError};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;

pub enum PromptResult<T> {
    Commands(Vec<TopLevelCommand<String>>),
    EOF,
    Error(ParseError::<T>),
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
    pub fn next(&self) -> Result<Vec<TopLevelCommand<String>>, ParseError<String>> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut line = String::new();

        let _ = stdout.write(self.ps1.as_bytes());
        let _ = stdout.flush();
        let mut state = Err(ParseError::Custom("".to_string()));
        while state.is_err() {
            state = match stdin.read_line(&mut line) {
                Ok(0) => return Err(ParseError::Custom("EOF".to_string())),
                Err(x) => Err(ParseError::Custom(x.to_string())),
                _ => {
                    let lexer = Lexer::new(line.chars());
                    Ok(DefaultParser::new(lexer)
                        .into_iter()
                        .collect::<Result<Vec<_>, _>>())
                }
            }
        }

        match state {
            Ok(x) => match x {
                Ok(x) => Ok(x),
                Err(x) => Err(ParseError::Custom(x.to_string())),
            },
            Err(_) => Err(ParseError::Custom("".to_string())),
        }
    }
}

// TODO handle single open " or ' at the end and maybe || && \ and |
// fn tokenize_line(line: &str) -> Result<Vec<Token>, ()> {
//     let mut tokens = Vec::new();
//     let mut in_double_quote = false;
//     let mut in_single_quote = false;
//     let mut escaped_character = false;

//     let mut token_content = String::new();

//     for ch in line.chars() {
//         if ch.is_whitespace() && !in_double_quote && !in_single_quote {
//             tokens.push(Token::Simple(token_content));
//             token_content = String::new();
//         } else if ch == '"' && in_double_quote && !in_single_quote && !escaped_character {
//             tokens.push(Token::DoubleQuotes(token_content));
//             token_content = String::new();
//             in_double_quote = false;
//         } else if ch == '\'' && in_single_quote && !in_double_quote && !escaped_character {
//             tokens.push(Token::Simple(token_content));
//             token_content = String::new();
//             in_single_quote = true;
//         } else if ch == '"' && !in_double_quote && !in_single_quote && !escaped_character {
//             in_double_quote = true;
//         } else if ch == '\'' && !in_single_quote && !in_double_quote && !escaped_character {
//             in_single_quote = true;
//         } else if ch != '\\' {
//             token_content.push(ch);
//         }

//         escaped_character = ch == '\\';
//     }

//     if !token_content.is_empty() {
//         tokens.push(Token::Simple(token_content));
//     }

//     if in_double_quote || in_single_quote {
//         return Err(())
//     }
//     if !tokens.is_empty() {
//         match tokens.last().unwrap() {
//             Token::Simple(inside) => {
//                 if inside.ends_with("|") || inside.ends_with("&&") || inside.ends_with("||") {
//                     return Err(());
//                 }
//             },
//             _ => (),
//         }
//     }

//     Ok(tokens)
// }
