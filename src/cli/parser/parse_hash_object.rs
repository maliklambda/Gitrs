use std::collections::VecDeque;

use crate::{
    cli::{ParseCliError, Token},
    command::Command,
    constants::{
        const_cmds::flags::{TYPE_FLAG_L, TYPE_FLAG_S, WRITE_FLAG_L, WRITE_FLAG_S},
        object_types::{BLOB_STR, COMMIT_STR, TREE_STR},
    },
    internals::objects::ObjectType,
};

pub fn parse_hash_object<'a>(
    tokens: &mut VecDeque<Token<'a>>,
) -> Result<Command<'a>, ParseCliError<'a>> {
    let value = {
        if let Token::TString(val) = tokens
            .pop_front()
            .ok_or(ParseCliError::MissingArgument("Value"))?
        {
            val
        } else {
            return Err(ParseCliError::InvalidValue {
                arg_name: "Value",
                value: tokens.pop_front().unwrap(),
            });
        }
    };

    // TODO: add parsing for type and write
    let (tp, write) = parse_hash_object_flags(tokens)?;

    assert!(tokens.is_empty(), "Non-empty token vector: {:?}", tokens);

    Ok(Command::HashObject { tp, value, write })
}

fn parse_hash_object_flags<'a>(
    tokens: &mut VecDeque<Token<'a>>,
) -> Result<(ObjectType, bool), ParseCliError<'a>> {
    let mut tp: Option<ObjectType> = None;
    let mut write = false;
    while let Some(t) = tokens.pop_front() {
        if let Token::TFlag(flag) = t {
            match flag {
                WRITE_FLAG_S | WRITE_FLAG_L => write = true,
                TYPE_FLAG_S | TYPE_FLAG_L => {
                    if let Some(Token::TString(type_str)) = tokens.pop_front() {
                        match type_str {
                            BLOB_STR => tp = Some(ObjectType::Blob),
                            TREE_STR => tp = Some(ObjectType::Tree),
                            COMMIT_STR => tp = Some(ObjectType::Commit),
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
                _ => return Err(ParseCliError::UnknownFlag(flag.to_string())),
            }
        } else {
            return Err(ParseCliError::InvalidToken(t));
        }
    }
    Ok((tp.unwrap_or(ObjectType::Blob), write))
}
