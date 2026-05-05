use log::info;

use crate::{command::Command, gitrs::Gitrs};

#[derive(Debug)]
pub enum ExecuteError {}

pub fn execute<'a>(cmd: Command<'a>) -> Result<(), ExecuteError> {
    info!("Executing command: {:?}", cmd);
    match cmd {
        Command::Init { default_branch: _ } => Gitrs::init_new(cmd).unwrap(),
        _ => todo!(),
    }
    todo!()
}
