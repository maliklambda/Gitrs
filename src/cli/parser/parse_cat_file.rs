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

    let values = parse_values(lexer)?;
    if values.is_empty() {
        return Err(ParseCliError::MissingArgument("Hash Value"));
    }

    Ok(Command::CatFile(CatFileConfig::new(values, flags)))
}

/// Parse flags for cat-file command.
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

/// Parse values for cat-file command.
/// Values are hashes separated by spaces.
/// Example:
///     $ gitrs cat-file -p <hash_1> <hash_2> ... <hash_n>
fn parse_values<'a>(lexer: &mut Lexer<'a>) -> Result<Vec<CommitHash>, ParseCliError<'a>> {
    let values: Result<Vec<CommitHash>, ParseCliError<'a>> = lexer
        .tokens
        .iter()
        .map(|t| parse_hash_value(t, CMD_CAT_FILE))
        .into_iter()
        .collect();
    values
}

/// parses a single hash value.
/// Check out fn parse_values() for more details.
fn parse_hash_value<'a>(
    t: &Token<'a>,
    cmd_str: &'static str,
) -> Result<CommitHash, ParseCliError<'a>> {
    let value = {
        if let Token::TString(val) = t {
            val
        } else {
            return Err(ParseCliError::InvalidValue {
                arg_name: "Value",
                value: *t,
            });
        }
    };

    Ok(CommitHash {
        hash: value
            .parse()
            .map_err(|_| ParseCliError::InvalidArgumentForCommand {
                cmd: cmd_str,
                invalid_arg: Token::TString(value),
            })?,
    })
}
