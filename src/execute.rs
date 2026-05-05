use log::info;

use crate::{command::Command, gitrs::Gitrs};

#[derive(Debug)]
pub enum ExecuteError {
    InitError { msg: String },
}

pub fn execute<'a>(cmd: Command<'a>) -> Result<(), ExecuteError> {
    info!("Executing command: {:?}", cmd);
    match cmd {
        Command::Init { default_branch } => {
            Gitrs::init_new(cmd).unwrap();
            info!("Initialization successfull. You are now on branch '{default_branch}'");
        }
        _ => todo!(),
    }
    Ok(())
}
