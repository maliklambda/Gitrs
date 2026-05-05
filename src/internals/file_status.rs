#[derive(Debug)]
pub enum FileStatus {
    Untracked,
    Unmodified,
    Modified,
    Staged,
}
