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
pub enum ParseCliError<'a> {
    InvalidToken(Token<'a>),
    UnknownFlag(String),
    MissingCommand,
    MissingArgument(&'static str),
    MissingValueForArg {
        arg: Token<'a>,
    },
    InvalidCommand(String),
    InvalidArgumentForCommand {
        cmd: &'static str,
        invalid_arg: Token<'a>,
    },
    InvalidValue {
        arg_name: &'static str,
        value: Token<'a>,
    },
    TooManyArguments,
}
