use crate::errors::MacroError;
use std::fs;

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Token {
    NIL,
    Identifier(String),
    Variable(String),
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}

pub fn read_file(path: &str) -> Result<String, MacroError> {
    fs::read_to_string(path).map_err(|e| MacroError::UnableToReadFile {
        path: path.to_string(),
        source: e,
    })
}

pub fn get_file_lines<'a>(text: &'a String) -> Vec<&'a str> {
    text.lines()
        .map(|x| {
            x.split_once("//")
                .map(|(before, _)| before)
                .unwrap_or(x)
                .trim()
        })
        .filter(|l| !l.is_empty())
        .collect()
}

pub fn get_line_tokens(lines: &[&str]) -> Vec<Vec<Token>> {
    lines
        .iter()
        .map(|l| {
            let mut split = l.split_whitespace();
            let command = get_token_from_text(split.next().unwrap());
            let rest = split.collect::<Vec<&str>>().join(" ");
            let args = rest
                .split(", ")
                .map(|x| get_token_from_text(x.trim()))
                .collect::<Vec<Token>>();

            [vec![command], args].concat()
        })
        .collect()
}

pub fn get_token_from_text(text: &str) -> Token {
    if let Ok(x) = text.parse::<i32>() {
        return Token::Int(x);
    } else if let Ok(x) = text.parse::<f32>() {
        return Token::Float(x);
    } else if let Ok(x) = text.parse::<bool>() {
        return Token::Bool(x);
    } else if text == "nil" {
        return Token::NIL;
    }

    if text.starts_with('"') && text.ends_with('"') {
        return Token::String(text[1..text.len() - 1].to_string());
    }

    if text.starts_with("$") {
        return Token::Variable(text[1..].to_string());
    }

    Token::Identifier(text.to_string())
}
