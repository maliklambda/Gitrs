pub const CLI_FLAG_PREFIX_SHORT: &str = "-";
pub const CLI_FLAG_PREFIX_LONG: &str = "--";
pub const SPACE: char = ' ';
pub const EQUAL: char = '=';
pub const NULL_HASH: &str = "NULL"; // TODO: change this to a real hash
pub const DEFAULT_BRANCH: &str = "main";
pub const BASE_DIR_NAME: &str = "gitrs";
pub const CONFIG_FILE: &str = "gitrsconfig";

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
);

register_const_mod!(
    flags,
    ALL_FLAG_S = "-a",
    ALL_FLAG_L = "--all",
    MESSAGE_FLAG_S = "-m",
    MESSAGE_FLAG_L = "--message",
);
