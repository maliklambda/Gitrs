#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandFlag<'a> {
    /// -a or --all
    All,

    /// -m or --message
    Message { msg: &'a str },
}

impl<'a> CommandFlag<'a> {}
