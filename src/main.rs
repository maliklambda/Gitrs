use crate::{cli::parser::Parser, execute::execute};

pub mod cli;
pub mod command;
pub mod config;
pub mod constants;
pub mod execute;
pub mod gitrs;
pub mod internals;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let args: Vec<String> = std::env::args().collect();
    let args = &args[2..];
    let mut parser = Parser::new(args).unwrap();
    let cmd = parser.parse().unwrap();
    execute(cmd).unwrap()
}
