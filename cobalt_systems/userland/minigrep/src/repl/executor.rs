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
        "edit" => {
            if cmd.args.is_empty() {
                Err("Usage: edit <file>".into())
            } else {
                crate::editor::open_editor(&cmd.args[0])
            }
        }
        "touch" => {
            if cmd.args.is_empty() {
                Err("Usage: touch <file>".into())
            } else {
                Ok(cmd_touch(&cmd.args[0]))
            }
        }
        "mkdir" => {
            if cmd.args.is_empty() {
                Err("Usage: mkdir <dir>".into())
            } else {
                Ok(cmd_mkdir(&cmd.args[0]))
            }
        }
        "rm" => {
            if cmd.args.is_empty() {
                Err("Usage: rm <path>".into())
            } else {
                Ok(cmd_rm(&cmd.args[0]))
            }
        }
        "clear" => Ok(cmd_clear()),
        "date" => Ok(cmd_date()),
        "whoami" => Ok(cmd_whoami()),
        "uname" => Ok(cmd_uname()),
        "echo" => Ok(cmd_echo(&cmd.args)),
        "exit" => cmd_exit(),
        _ => Err(format!("Unknown command: {}", cmd.name)),
    }
}
