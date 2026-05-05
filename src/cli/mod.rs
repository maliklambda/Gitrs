mod flags;
mod lexer;
pub mod parser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    TNumber(u32),
    TString(&'a str),
    TFlag(&'a str),
    TCommand(&'a str),
    TEOF,
}

#[derive(Debug)]
pub enum ParseCliError {
    InvalidToken(String),
    UnknownFlag(String),
    MissingCommand,
    InvalidCommand(String),
    InvalidArgument {
        cmd: &'static str,
        invalid_arg: String,
    },
}
