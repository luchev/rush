use conch_parser::lexer::Lexer;
use conch_parser::parse::{DefaultParser, ParseError};
use conch_parser::ast::TopLevelCommand;
use std::io;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum Token {
    DoubleQuotes(String),
    Simple(String),
}

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

impl Prompt {
    pub fn new() -> Prompt {
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

    pub fn next(&self) -> Result<Vec<TopLevelCommand<String>>, ParseError<()>>{
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut line = String::new();

        stdout.write(self.ps1.as_bytes());
        stdout.flush();
        let mut state = Err(ParseError::Custom(()));
        while state.is_err() {
            state = match stdin.read_line(&mut line) {
                Ok(0) => return Err(ParseError::Custom(())),
                Err(x) => Err(ParseError::Custom(())),
                _ => {
                    let lexer = Lexer::new(line.chars());
                    Ok(DefaultParser::new(lexer).into_iter().collect::<Result<Vec<_>,_>>())
                }
            }
        }

        match state {
            Ok(x) => Ok(x.unwrap()),
            Err(_) => Err(ParseError::Custom(())),
        }
    }

    pub fn next_instruction(&self) -> Result<Vec<Token>, ()> {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut line = String::new();

        let mut state = Err(());
        stdout.write(self.ps1.as_bytes());
        stdout.flush();
        while state.is_err() {
            state = match stdin.read_line(&mut line) {
                Ok(0) => return Err(()),
                Err(_) => Err(()),
                _ => {
                    let line = line.trim();
                    tokenize_line(&line)
                }
            }
        }

        state
    }
}

// TODO handle single open " or ' at the end and maybe || && \ and | 
fn tokenize_line(line: &str) -> Result<Vec<Token>, ()> {
    let mut tokens = Vec::new();
    
    let mut in_double_quote = false;
    let mut in_single_quote = false;
    let mut escaped_character = false;

    let mut token_content = String::new();

    for ch in line.chars() {
        if ch.is_whitespace() && !in_double_quote && !in_single_quote {
            tokens.push(Token::Simple(token_content));
            token_content = String::new();
        } else if ch == '"' && in_double_quote && !in_single_quote && !escaped_character {
            tokens.push(Token::DoubleQuotes(token_content));
            token_content = String::new();
            in_double_quote = false;
        } else if ch == '\'' && in_single_quote && !in_double_quote && !escaped_character {
            tokens.push(Token::Simple(token_content));
            token_content = String::new();
            in_single_quote = true;
        } else if ch == '"' && !in_double_quote && !in_single_quote && !escaped_character {
            in_double_quote = true;
        } else if ch == '\'' && !in_single_quote && !in_double_quote && !escaped_character {
            in_single_quote = true;
        } else if ch != '\\' {
            token_content.push(ch);
        }

        escaped_character = ch == '\\';
    }

    if !token_content.is_empty() {
        tokens.push(Token::Simple(token_content));
    }

    if in_double_quote || in_single_quote {
        return Err(())
    }
    
    if !tokens.is_empty() {
        match tokens.last().unwrap() {
            Token::Simple(inside) => {
                if inside.ends_with("|") || inside.ends_with("&&") || inside.ends_with("||") {
                    return Err(());
                }
            },
            _ => (),
        }
    }

    Ok(tokens)
}
