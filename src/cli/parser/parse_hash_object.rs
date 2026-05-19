use log::debug;

use crate::{
    cli::{ParseCliError, Token, lexer::Lexer},
    command::Command,
    constants::{
        const_cmds::flags::{TYPE_FLAG_L, TYPE_FLAG_S, WRITE_FLAG_L, WRITE_FLAG_S},
        object_types::{BLOB_STR, COMMIT_STR, TREE_STR},
    },
    internals::{
        hash::{HashObjectConfig, HashObjectFlags},
        objects::ObjectType,
    },
};

pub fn parse_hash_object<'a, 'b>(lexer: &'b mut Lexer<'a>) -> Result<Command<'a>, ParseCliError<'b>>
where
    'b: 'a,
{
    let ho_flags = parse_hash_object_flags(lexer)?;
    debug!("Hash object tokens after parsing flags: {:?}", lexer.tokens);

    assert_eq!(
        lexer.tokens.len(),
        1,
        "Non-empty token vector (before value extraction): {:?}",
        lexer.tokens
    );
    let value = extract_value(lexer)?;

    Ok(Command::HashObject(HashObjectConfig::new(value, ho_flags)))
}

fn parse_hash_object_flags<'a>(
    lexer: &mut Lexer<'a>,
) -> Result<HashObjectFlags, ParseCliError<'a>> {
    let mut tp: Option<ObjectType> = None;
    let mut write = false;

    while let Some(Token::TFlag(flag)) = lexer.peek() {
        match flag {
            WRITE_FLAG_S | WRITE_FLAG_L => {
                lexer.next();
                write = true;
            }
            TYPE_FLAG_S | TYPE_FLAG_L => {
                lexer.next();
                tp = Some(parse_type_flag(lexer, flag)?);
            }
            _ => return Err(ParseCliError::UnknownFlag(flag)),
        }
    }

    Ok(HashObjectFlags::new(tp.unwrap_or(ObjectType::Blob), write))
}

fn extract_value<'b, 'a>(lexer: &'b mut Lexer<'a>) -> Result<&'a str, ParseCliError<'a>> {
    if let Some(Token::TString(val)) = lexer.peek() {
        Ok(val)
    } else {
        Err(ParseCliError::InvalidValue {
            arg_name: "Value",
            value: Token::TString("Add this here"),
        })
    }
}

fn parse_type_flag<'a>(
    lexer: &mut Lexer<'a>,
    flag: &'a str,
) -> Result<ObjectType, ParseCliError<'a>> {
    if let Some(Token::TString(type_str)) = lexer.next() {
        match type_str {
            BLOB_STR => Ok(ObjectType::Blob),
            TREE_STR => Ok(ObjectType::Tree),
            COMMIT_STR => Ok(ObjectType::Commit),
            _ => {
                return Err(ParseCliError::InvalidValue {
                    arg_name: if flag == TYPE_FLAG_L {
                        TYPE_FLAG_L
                    } else {
                        TYPE_FLAG_S
                    },
                    value: Token::TString(type_str),
                });
            }
        }
    } else {
        return Err(ParseCliError::MissingValueForArg {
            arg: Token::TFlag(flag),
        });
    }
}
