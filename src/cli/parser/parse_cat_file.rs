use crate::{
    cli::{ParseCliError, Token, lexer::Lexer},
    command::Command,
    constants::const_cmds::{
        flags::{PRETTY_PRINT_FLAG_S, SIZE_FLAG_S, TYPE_FLAG_S},
        keywords::CMD_CAT_FILE,
    },
    internals::{
        hash::commit_hash::CommitHash,
        objects::cat_file::{CatFileConfig, CatFileMode},
    },
};

pub fn parse_cat_file<'a, 'b>(lexer: &'b mut Lexer<'a>) -> Result<Command<'a>, ParseCliError<'b>>
where
    'b: 'a,
{
    let flags = parse_cat_file_flags(lexer)?;

    let value = {
        if let Token::TString(val) = lexer
            .peek()
            .ok_or(ParseCliError::MissingArgument("Value"))?
        {
            val
        } else {
            return Err(ParseCliError::InvalidValue {
                arg_name: "Value",
                value: lexer.next().unwrap(),
            });
        }
    };

    let value = CommitHash {
        hash: value
            .parse()
            .map_err(|_| ParseCliError::InvalidArgumentForCommand {
                cmd: CMD_CAT_FILE,
                invalid_arg: Token::TString(value),
            })?,
    };

    Ok(Command::CatFile(CatFileConfig::new(value, flags)))
}

fn parse_cat_file_flags<'a>(lexer: &mut Lexer<'a>) -> Result<CatFileMode, ParseCliError<'a>> {
    if let Some(Token::TFlag(flag)) = lexer.next() {
        match flag {
            TYPE_FLAG_S => Ok(CatFileMode::Type),
            PRETTY_PRINT_FLAG_S => Ok(CatFileMode::PrettyPrint),
            SIZE_FLAG_S => Ok(CatFileMode::Size),
            _ => Err(ParseCliError::UnknownFlag(flag)),
        }
    } else {
        Err(ParseCliError::MissingArgument(
            "Cat file flag (-p, -t, ...)",
        ))
    }
}
