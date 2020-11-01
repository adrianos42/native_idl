pub mod idl;
pub mod ids;

pub const INDENT: &str = "    ";
pub const MAX_LENGTH: usize = 100;
pub const COMMENT_START: &str = "--";

pub(crate) const NEW_LINE: &str = "\n\n";
pub(crate) const CLOSE_NEW_LINE: &str = "}\n\n";
pub(crate) const OPEN_NEW_LINE: &str = " {\n";