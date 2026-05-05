/// All allowed commands are specified here
#[derive(Debug, Clone, PartialEq)]
pub enum Command<'a> {
    /// Initialize a gitrs project
    Init { default_branch: &'a str },

    /// Display status of currently tracked & untracked files.
    Status,

    /// Add file(s) to staging
    Add { files: Vec<&'a str> },

    /// Commit currently staged files
    Commit { message: &'a str },

    /// Display a log of previous commits
    Log,

    /// Remove a file from staging.
    Reset { files: Vec<&'a str> },
}
