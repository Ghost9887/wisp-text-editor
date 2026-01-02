use termion::{raw::IntoRawMode, clear, cursor}; 
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}",  clear::All).unwrap();
    write!(stdout, "{}", cursor::Goto(10, 10)).unwrap();
    write!(stdout, "Hello, World!").unwrap();
    let _ = stdout.flush();
    thread::sleep(Duration::from_secs(5));
}
