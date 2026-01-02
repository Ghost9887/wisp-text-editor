use termion::{raw::IntoRawMode, clear, cursor, terminal_size}; 
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::process;

struct Global {
    terminal_w: u16,
    terminal_h: u16,
}
impl Global {
    fn new() -> Self {
        Global {
            terminal_w: 0,
            terminal_h: 0,
        }
    }
    fn update_terminal_size(&mut self) {
        let (w, h) = terminal_size().unwrap_or((20, 20));
        self.terminal_w = w;
        self.terminal_h = h;
    }
}

fn print_tui(stdout: &mut io::Stdout, global: &Global) {
    write!(stdout, "{}",  clear::All).unwrap();
    write!(stdout, "{}", cursor::Goto(global.terminal_w / 2, global.terminal_h / 2)).unwrap();
    write!(stdout, "Hello, World!").unwrap();
    stdout.flush().unwrap();
}

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap_or_else(|error| {
        eprintln!("Failed to go into Raw Mode: {error}");
        process::exit(1);
    });

    let mut global = Global::new(); 
    global.update_terminal_size();

    print_tui(&mut stdout, &global);
    thread::sleep(Duration::from_secs(5));
}
