use termion::{raw::IntoRawMode, clear, cursor, terminal_size, input::TermRead, event::Key}; 
use std::io::{self, Write};
use std::process;
use std::cmp;

//size of the bottom bar
const BOTTOM_BAR: u16 = 1;

#[derive(Debug)]
enum Mode {
    Insert,
    Normal,
}

enum Command {
    Invalid,
    EnterNormalMode,
    EnterInsertMode,
    InsertChar(char),
    DeleteChar,
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
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
    mode: Mode,
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
            mode: Mode::Normal,
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
    fn current_line(&mut self) -> &mut Line {
        &mut self.lines[self.cur_row as usize - 1]   
    }
}

fn print_tui(stdout: &mut io::Stdout, global: &Global) {
    write!(stdout, "{}",  clear::All).unwrap();

    write!(stdout, "{}{}:{}", cursor::Goto(global.terminal_w - 5, global.terminal_h), 
    global.cur_row, global.cur_col).unwrap();
    write!(stdout, "{}{:?}", cursor::Goto(5, global.terminal_h), global.mode).unwrap();

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

fn parse_command(key: Key, global: &Global) -> Command {
    match global.mode {
        Mode::Normal => {
            match key {
                Key::Char('a') => Command::EnterInsertMode,
                Key::Char('j') => Command::MoveDown,
                Key::Char('k') => Command::MoveUp,
                Key::Char('l') => Command::MoveRight,
                Key::Char('h') => Command::MoveLeft,
                Key::Char('q') => Command::Quit,
                _ => Command::Invalid,
            }
        },
        Mode::Insert => {
            match key {
                Key::Char(c) => Command::InsertChar(c),
                Key::Esc => Command::EnterNormalMode,
                Key::Backspace => Command::DeleteChar,
                _ => Command::Invalid,
            }
        },
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
        
        let cmd = parse_command(key, &global);

        match cmd {
            Command::Quit => break,
            Command::EnterNormalMode => global.mode = Mode::Normal,
            Command::EnterInsertMode => global.mode = Mode::Insert,
            Command::MoveUp => {
                if global.cur_row > 1 {
                    global.cur_row -= 1;   
                }
            },
            Command::MoveDown => {
                if (global.cur_row as usize + 1) < global.lines.len() {
                    global.cur_row += 1;
                }
            },
            Command::MoveRight => {
                if (global.cur_col as usize) < global.current_line().chars.len() + 1 {
                    global.cur_col += 1;
                }
            },
            Command::MoveLeft => {
                if global.cur_col > global.line_numbers - 1 {
                    global.cur_col -= 1;
                }
            },
            Command::InsertChar(c) => {
                let cur_col = global.cur_col;
                global.current_line().chars.insert(cur_col as usize - 1, c);
                global.cur_col += 1;
            },
            Command::DeleteChar => {
                if global.cur_col > global.line_numbers - 1 {
                    global.cur_col -= 1;
                    let cur_col = global.cur_col;
                    global.current_line().chars.remove(cur_col as usize - 1);
                }
            }
            Command::Invalid => {},
        }
        
    }
}

