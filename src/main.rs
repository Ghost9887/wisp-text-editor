use termion::{raw::IntoRawMode, clear, cursor, terminal_size, input::TermRead, event::Key}; 
use std::io::{self, Write};
use std::process;
use std::cmp;

//size of the bottom bar
const BOTTOM_BAR: u16 = 1;

enum Command {
    Invalid,
    InsertChar(char),
    Quit,
}

struct Line {
    chars: Vec<char>,
}
impl Line {
    fn new() -> Self {
        Line{
            chars: Vec::new(),
        }
    }
}

struct Global {
    terminal_w: u16,
    terminal_h: u16,
    cur_row: u16,
    cur_col: u16,
    line_numbers: u16,
    lines: Vec<Line>,
}
impl Global {
    fn new() -> Self {
        Global {
            terminal_w: 0,
            terminal_h: 0,
            cur_row: 1,
            cur_col: 1,
            line_numbers: 0,
            lines: Vec::new(),
        }
    }
    fn update_terminal_size(&mut self) {
        let (w, h) = terminal_size().unwrap_or((20, 20));
        self.terminal_w = w;
        self.terminal_h = h;
    }
    fn update_line_numbers(&mut self) {
       let size = self.lines.len().to_string();
       self.line_numbers = size.len() as u16 + 1;
    }
    fn new_line(&mut self, line: Line) {
        self.lines.push(line);
    }
}

fn print_tui(stdout: &mut io::Stdout, global: &Global) {
    write!(stdout, "{}",  clear::All).unwrap();

    write!(stdout, "{}{}:{}", cursor::Goto(global.terminal_w - 5, global.terminal_h), 
    global.cur_row, global.cur_col).unwrap();

    print_line_nubmers(stdout, global);
    print_content(stdout, global);

    write!(stdout, "{}", cursor::Goto(global.cur_col + global.line_numbers, global.cur_row)).unwrap();
    stdout.flush().unwrap();
}

fn print_line_nubmers(stdout: &mut io::Stdout, global: &Global) {
    let max = cmp::min(global.lines.len(), global.terminal_h as usize - BOTTOM_BAR as usize);
    for i in 0..max {
        write!(stdout, "{}{}|", cursor::Goto(1, i as u16 + 1), i + 1).unwrap();  
    }
}

fn print_content(stdout: &mut io::Stdout, global: &Global) {
    for (i, line) in global.lines.iter().enumerate() {
        for (j, char) in line.chars.iter().enumerate() {
            write!(stdout, "{}{char}", cursor::Goto(j as u16 + 1 + global.line_numbers, i as u16 + 1)).unwrap();
        }
    }
}

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap_or_else(|error| {
        eprintln!("Failed to go into Raw Mode: {error}");
        process::exit(1);
    });

    let mut global = Global::new(); 
    global.new_line(Line::new());
    loop {

        global.update_terminal_size();
        global.update_line_numbers();
        print_tui(&mut stdout, &global);

        let key = match io::stdin().keys().next(){
            Some(key) => {
                match key {
                    Ok(key) => key,
                    Err(_) => continue,
                }
            },
            None => continue,
        };

        let cmd = match key {
            Key::Char('q') => Command::Quit,
            Key::Char(c) => Command::InsertChar(c),
            _ => Command::Invalid,
        };

        match cmd {
            Command::Quit => break,
            Command::InsertChar(c) => {
                global.lines[0].chars.push(c);
                global.cur_col += 1;
            },
            Command::Invalid => {},
        }
        
    }
}

