use crate::commands::{dir::*, file::*, system::*};
use crate::repl::parser::Command;

pub fn execute(cmd: Command) -> Result<(), String> {
    match cmd.name.as_str() {
        "pwd" => Ok(cmd_pwd()),
        "ls" => Ok(cmd_ls()),
        "cd" => {
            if cmd.args.is_empty() {
                Err("Usage: cd <directory>".into())
            } else {
                Ok(cmd_cd(&cmd.args[0]))
            }
        }
        "cat" => {
            if cmd.args.is_empty() {
                Err("Usage: cat <file>".into())
            } else {
                Ok(cmd_cat(&cmd.args[0]))
            }
        }
<<<<<<< HEAD
=======
        "edit" => {
            if cmd.args.is_empty() {
                Err("Usage: edit <file>".into())
            } else {
                crate::editor::open_editor(&cmd.args[0])
            }
        }
>>>>>>> 18ecb77
        "touch" => Ok(cmd_touch(&cmd.args[0])),
        "mkdir" => Ok(cmd_mkdir(&cmd.args[0])),
        "rm" => Ok(cmd_rm(&cmd.args[0])),
        "clear" => Ok(cmd_clear()),
        "exit" => cmd_exit(),
        _ => Err(format!("Unknown command: {}", cmd.name)),
    }
}
