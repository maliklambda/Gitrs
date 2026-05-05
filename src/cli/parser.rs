use crate::{
    cli::{ParseCliError, Token, flags::CommandFlag, lexer::Lexer},
    command::Command,
    constants::{DEFAULT_BRANCH, keywords::*},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, ParseCliError> {
        Ok(Self {
            lexer: Lexer::new(args)?,
        })
    }

    pub fn parse(&mut self) -> Result<Command<'_>, ParseCliError> {
        if let Token::TCommand(cmd) = self.lexer.next() {
            return match cmd {
                CMD_STATUS => self.parse_status(),
                CMD_ADD => self.parse_add(),
                CMD_COMMIT => self.parse_commit(),
                CMD_LOG => self.parse_log(),
                CMD_RESET => self.parse_reset(),
                CMD_INIT => self.parse_init(),
                _ => Err(ParseCliError::InvalidCommand(cmd.to_string())),
            };
        }
        Err(ParseCliError::MissingCommand)
    }

    fn parse_status(&mut self) -> Result<Command<'_>, ParseCliError> {
        todo!()
    }

    fn parse_add(&mut self) -> Result<Command<'_>, ParseCliError> {
        let mut file_names: Vec<&str> = vec![];
        let mut flags: Vec<CommandFlag> = vec![];
        todo!()
    }

    fn parse_log(&mut self) -> Result<Command<'_>, ParseCliError> {
        todo!()
    }

    fn parse_reset(&mut self) -> Result<Command<'_>, ParseCliError> {
        todo!()
    }

    fn parse_commit(&mut self) -> Result<Command<'_>, ParseCliError> {
        todo!()
    }

    fn parse_init(&mut self) -> Result<Command<'_>, ParseCliError> {
        if !self.lexer.tokens.is_empty() {
            return Err(ParseCliError::InvalidArgument {
                cmd: CMD_INIT,
                invalid_arg: format!("{:?}", self.lexer.next()),
            });
        }
        Ok(Command::Init {
            default_branch: DEFAULT_BRANCH,
        })
    }
}
