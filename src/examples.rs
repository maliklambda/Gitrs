#![allow(dead_code)]

/// Examples of valid commands
/// Commands should be prefixed with "gitrs <CMD>" or (for development) "cargo run . <CMD>")
/// Variables are denoted by angle brackets ("<VAR>").
pub const EXAMPLE_COMMANDS: [&str; 3] = [
    "cat-file -p <hash>",
    "cat-file -t <hash>",
    "hash-object --write <obj>",
];
