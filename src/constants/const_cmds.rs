macro_rules! register_const_mod {
    ($mod_name:ident, $($name:ident = $val:expr),* $(,)?) => {
        pub mod $mod_name {
            // 1. Generate the individual constants
            $( pub const $name: &str = $val; )*

            // 2. Generate the array containing all of them
            pub const ALL: &[&str] = &[
                $( $name ),*
            ];
        }
    }
}

register_const_mod!(
    keywords,
    CMD_INIT = "init",
    CMD_STATUS = "status",
    CMD_ADD = "add",
    CMD_COMMIT = "commit",
    CMD_LOG = "log",
    CMD_RESET = "reset",
    CMD_HASH_FILE = "hash-file",
    CMD_BUILD_TREE = "build-tree",
    CMD_HASH_OBJECT = "hash-object",
    CMD_CAT_FILE = "cat-file",
);

register_const_mod!(
    flags,
    ALL_FLAG_S = "-a",
    ALL_FLAG_L = "--all",
    MESSAGE_FLAG_S = "-m",
    MESSAGE_FLAG_L = "--message",
    TYPE_FLAG_S = "-t",
    TYPE_FLAG_L = "--type",
    WRITE_FLAG_S = "-w",
    WRITE_FLAG_L = "--write",
    PRETTY_PRINT_FLAG_S = "-p",
    SIZE_FLAG_S = "-s",
);
