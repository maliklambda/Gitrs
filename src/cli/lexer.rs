use std::collections::VecDeque;

use log::debug;

use crate::{
    cli::{ParseCliError, Token},
    constants::{
        const_cmds::{flags, keywords},
        special_chars::{CLI_FLAG_PREFIX_SHORT, EQUAL, SPACE},
    },
};

pub struct Lexer<'a> {
    pub tokens: VecDeque<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, ParseCliError<'a>> {
        Ok(Self {
            tokens: Self::tokenize(args)?,
        })
    }

    pub fn next(&mut self) -> Token<'_> {
        self.tokens.pop_front().unwrap_or(Token::TEOF)
    }

    pub fn peek(&mut self) -> Token<'_> {
        self.tokens.iter().last().copied().unwrap_or(Token::TEOF)
    }

    /// Build tokens from raw arguments.
    fn tokenize(args: &'a [String]) -> Result<VecDeque<Token<'a>>, ParseCliError<'a>> {
        let mut tokens: VecDeque<Token> = VecDeque::new();
        for arg in args {
            let token = match arg {
                cmd if keywords::ALL.contains(&arg.as_str()) => {
                    debug!("Command: '{arg}'");
                    Token::TCommand(cmd)
                }
                arg if arg.contains(SPACE) => {
                    debug!("Raw string: '{arg}'");
                    Token::TString(arg)
                }
                arg if arg.contains(EQUAL) => {
                    todo!("Key-Value Pair");
                }
                flag if flag.starts_with(CLI_FLAG_PREFIX_SHORT)
                    & flags::ALL.contains(&flag.as_str()) =>
                {
                    debug!("Flag arg: {flag}");
                    Token::TFlag(flag)
                }
                _ if let Some(number) = arg.parse::<u32>().ok() => {
                    debug!("Number arg: {number}");
                    Token::TNumber(number)
                }
                _ => Token::TString(arg),
            };
            tokens.push_back(token)
        }
        debug!("Tokens (non-reversed): {:?}", tokens);
        // tokens.reverse(); // reverse tokens to enable usage of tokens.pop()
        Ok(tokens)
    }
}
