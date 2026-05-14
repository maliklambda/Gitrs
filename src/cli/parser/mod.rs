pub mod parse_hash_object;

use crate::{
    cli::{ParseCliError, Token, lexer::Lexer, parser::parse_hash_object::parse_hash_object},
    command::Command,
    constants::{DEFAULT_BRANCH, const_cmds::keywords::*},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, ParseCliError<'a>> {
        Ok(Self {
            lexer: Lexer::new(args)?,
        })
    }

    pub fn parse(&mut self) -> Result<Command<'_>, ParseCliError<'_>> {
        if let Token::TCommand(cmd) = self.lexer.next() {
            return match cmd {
                CMD_STATUS => self.parse_status(),
                CMD_ADD => self.parse_add(),
                CMD_COMMIT => self.parse_commit(),
                CMD_LOG => self.parse_log(),
                CMD_RESET => self.parse_reset(),
                CMD_INIT => self.parse_init(),
                CMD_HASH_FILE => self.parse_hash_file(),
                CMD_BUILD_TREE => self.parse_build_tree(),
                CMD_HASH_OBJECT => parse_hash_object(&mut self.lexer.tokens),
                _ => Err(ParseCliError::InvalidCommand(cmd.to_string())),
            };
        }
        Err(ParseCliError::MissingCommand)
    }

    fn parse_status(&mut self) -> Result<Command<'_>, ParseCliError<'_>> {
        if !self.lexer.tokens.is_empty() {
            return Err(ParseCliError::InvalidArgumentForCommand {
                cmd: CMD_STATUS,
                invalid_arg: self.lexer.next(),
            });
        }
        Ok(Command::Status)
    }

    fn parse_add(&mut self) -> Result<Command<'_>, ParseCliError<'_>> {
        todo!()
    }

    fn parse_log(&mut self) -> Result<Command<'_>, ParseCliError<'_>> {
        todo!()
    }

    fn parse_reset(&mut self) -> Result<Command<'_>, ParseCliError<'_>> {
        todo!()
    }

    fn parse_commit(&mut self) -> Result<Command<'_>, ParseCliError<'_>> {
        todo!()
    }

    fn parse_init(&mut self) -> Result<Command<'_>, ParseCliError<'_>> {
        if !self.lexer.tokens.is_empty() {
            return Err(ParseCliError::InvalidArgumentForCommand {
                cmd: CMD_INIT,
                invalid_arg: self.lexer.next(),
            });
        }
        Ok(Command::Init {
            default_branch: DEFAULT_BRANCH,
        })
    }

    fn parse_hash_file(&'_ mut self) -> Result<Command<'_>, ParseCliError<'_>> {
        if self.lexer.tokens.len() > 1 {
            return Err(ParseCliError::TooManyArguments);
        } else if self.lexer.tokens.is_empty() {
            return Err(ParseCliError::MissingArgument("Filename"));
        }
        Ok(Command::HashFile {
            filename: self.lexer.tokens.pop_front().unwrap(),
        })
    }

    fn parse_build_tree(&mut self) -> Result<Command<'_>, ParseCliError<'_>> {
        if !self.lexer.tokens.is_empty() {
            return Err(ParseCliError::TooManyArguments);
        }
        Ok(Command::BuildTree)
    }
}
