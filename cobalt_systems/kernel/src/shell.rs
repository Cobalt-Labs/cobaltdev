use crate::print;
use crate::println;
use spin::Mutex;

const MAX_COMMAND_LEN: usize = 128;

struct ShellState {
    buffer: [char; MAX_COMMAND_LEN],
    len: usize,
}

static SHELL: Mutex<ShellState> = Mutex::new(ShellState {
    buffer: ['\0'; MAX_COMMAND_LEN],
    len: 0,
});

pub fn init_shell() {
    print_prompt();
}

fn print_prompt() {
    print!("\n[cobalt@kernel /]# ");
}

pub fn handle_char(c: char) {
    let mut shell = SHELL.lock();
    match c {
        '\n' => {
            println!("");
            execute_command(&shell.buffer[..shell.len]);
            shell.len = 0;
            print_prompt();
        }
        '\x08' => { // Backspace
            if shell.len > 0 {
                shell.len -= 1;
                print!("\x08 \x08"); // Erase character from screen
            }
        }
        _ => {
            if shell.len < MAX_COMMAND_LEN {
                let len = shell.len;
                shell.buffer[len] = c;
                shell.len += 1;
                print!("{}", c);
            }
        }
    }
}

pub fn handle_raw_key(_key: pc_keyboard::KeyCode) {
    // Handle arrow keys etc if needed
}

fn execute_command(buffer: &[char]) {
    if buffer.is_empty() { return; }
    
    if compare_cmd(buffer, "help") {
        println!("Available commands: help, echo, clear, version");
    } else if compare_cmd(buffer, "clear") {
        crate::vga_buffer::WRITER.lock().clear_screen();
    } else if compare_cmd(buffer, "version") {
        println!("Cobalt Kernel v0.1.0");
    } else if buffer.len() >= 4 && &buffer[..4] == &['e', 'c', 'h', 'o'] {
        if buffer.len() > 5 && buffer[4] == ' ' {
            for &c in &buffer[5..] {
                print!("{}", c);
            }
        }
        println!("");
    } else {
        print!("Unknown command: ");
        for &c in buffer {
            print!("{}", c);
        }
        println!("");
    }
}

fn compare_cmd(buffer: &[char], target: &str) -> bool {
    if buffer.len() != target.chars().count() {
        return false;
    }
    buffer.iter().zip(target.chars()).all(|(a, b)| *a == b)
}
