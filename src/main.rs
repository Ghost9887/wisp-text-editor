use termion::{raw::IntoRawMode, clear, cursor, terminal_size, input::TermRead, event::Key}; 
use std::io::{self, Write};
use std::process;
use std::cmp;

//size of the bottom bar
const BOTTOM_BAR: u16 = 1;

#[derive(Debug, PartialEq)]
enum Mode {
    Insert,
    Normal,
    CommandMode, 
}

enum Command {
    Invalid,
    EnterNormalMode,
    EnterInsertMode,
    EnterCommandMode,
    NewLine,
    Tab,
    InsertChar(char),
    InsertCommandChar(char),
    DeleteChar,
    DeleteCharX,
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    Save,
    SaveQuit,
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
    command: String,
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
            command: String::new(),
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
    fn clear_command(&mut self) {
        self.command.clear();
    }
}

fn print_tui(stdout: &mut io::Stdout, global: &Global) {
    write!(stdout, "{}",  clear::All).unwrap();

    if global.mode != Mode::CommandMode {
        write!(stdout, "{}{}:{}", cursor::Goto(global.terminal_w - 5, global.terminal_h), 
        global.cur_row, global.cur_col).unwrap();
        write!(stdout, "{}{:?}", cursor::Goto(5, global.terminal_h), global.mode).unwrap();
    }else {
        print_command_content(stdout, global);
    }

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
            if *char == '\n'{
                write!(stdout, "{}N", cursor::Goto(j as u16 + 1 + global.line_numbers, i as u16 + 1)).unwrap();
            }else {
                write!(stdout, "{}{char}", cursor::Goto(j as u16 + 1 + global.line_numbers, i as u16 + 1)).unwrap();
            }
        }
    }
}

fn print_command_content(stdout: &mut io::Stdout, global: &Global) {
    write!(stdout, "{}:", cursor::Goto(1, global.terminal_h)).unwrap();
    write!(stdout, "{}{}", cursor::Goto(2, global.terminal_h), global.command).unwrap();
}

fn parse_terminal_command(t_cmd: &str) -> Command {
    match t_cmd {
        "w" => Command::Save,
        "q" => Command::Quit,
        "wq" => Command::SaveQuit,
        _ => Command::EnterNormalMode,
    }
}

fn parse_command(key: Key, global: &mut Global) -> Command {
    match global.mode {
        Mode::CommandMode => {
            match key {
                Key::Esc => {
                    global.clear_command();
                    Command::EnterNormalMode
                },
                Key::Backspace => {
                    global.command.pop();
                    Command::Invalid
                },
                Key::Char('\n') => {
                    let cmd: Command  = parse_terminal_command(&global.command);
                    global.clear_command();
                    cmd
                },
                Key::Char(c) => Command::InsertCommandChar(c),
                _ => Command::Invalid,
            }
        }
        Mode::Normal => {
            match key {
                Key::Char('a') => Command::EnterInsertMode,
                Key::Char('j') => Command::MoveDown,
                Key::Char('k') => Command::MoveUp,
                Key::Char('l') => Command::MoveRight,
                Key::Char('h') => Command::MoveLeft,
                Key::Char('x') => Command::DeleteCharX,
                Key::Char(':') => Command::EnterCommandMode,
                _ => Command::Invalid,
            }
        },
        Mode::Insert => {
            match key {
                Key::Char('\n') => Command::NewLine,
                Key::Char('\t') => Command::Tab,
                Key::Char(c) => Command::InsertChar(c),
                Key::Esc => Command::EnterNormalMode,
                Key::Backspace => Command::DeleteChar,
                _ => Command::Invalid,
            }
        },
    }
}

//TODO: implement
fn save_file(_global: &Global) -> Result<(), String> {
    Ok(())
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
        
        let cmd = parse_command(key, &mut global);

        match cmd {
            Command::Quit => break,
            Command::EnterNormalMode => global.mode = Mode::Normal,
            Command::EnterInsertMode => global.mode = Mode::Insert,
            Command::EnterCommandMode => global.mode = Mode::CommandMode,
            Command::InsertCommandChar(c) => global.command.push(c),
            Command::Save => {
                match save_file(&global) {
                    Ok(()) => {
                    },
                    Err(e) => {
                        global.command.clear();
                        global.command.push_str(&e);
                    },
                }
                global.mode = Mode::Normal;
            },
            Command::SaveQuit => {
                match save_file(&global) {
                    Ok(()) => {
                        break;
                    },
                    Err(e) => {
                        global.command.clear();
                        global.command.push_str(&e);
                    },
                }
                global.mode =Mode::Normal;
            },
            Command::MoveUp => {
                if global.cur_row > 1 {
                    global.cur_row -= 1;   
                    if global.current_line().chars.is_empty() {
                        global.cur_col = 1;
                    }
                    else if global.current_line().chars.len() < (global.cur_col as usize) {
                        global.cur_col = global.current_line().chars.len() as u16 + 1;
                    }
                }
            },
            Command::MoveDown => {
                if (global.cur_row as usize) < global.lines.len() {
                    global.cur_row += 1;
                    if global.current_line().chars.is_empty() {
                        global.cur_col = 1;
                    }
                    else if global.current_line().chars.len() < (global.cur_col as usize) {
                        global.cur_col = global.current_line().chars.len() as u16 + 1;
                    }
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
            Command::Tab => {
                for _ in 0..4 {
                    let cur_col = global.cur_col;
                    global.current_line().chars.insert(cur_col as usize - 1, ' ');
                    global.cur_col += 1;
                }
            }
            //MIGHT NEED TO FIX LATER WE'LL SEE
            Command::NewLine => {
                //write the \n char into the vec
                let cur_col = global.cur_col;
                global.current_line().chars.insert(cur_col as usize - 1, '\n');
                global.cur_col += 1;

                let cur_col = global.cur_col;
                //at the end of the current line
                if (cur_col as usize) >= global.current_line().chars.len() {
                    let cur_row = global.cur_row;
                    global.lines.insert(cur_row as usize, Line::new());
                    global.cur_row += 1;
                    global.cur_col = 1;
                }
                //at the start of the current line
                else if cur_col == 1 {
                    let cur_row = global.cur_row;
                    global.lines.insert(cur_row as usize - 1, Line::new()); 
                    global.cur_row += 1;
                }
                //in the current line
                else {
                    let cur_row = global.cur_row;
                    global.lines.insert(cur_row as usize, Line::new());
                    global.cur_row += 1;
                    global.current_line().chars = global.lines[cur_row as usize - 1].chars.split_off(cur_col as usize - 1);
                    global.cur_col = 1;
                }

            }
            Command::DeleteChar => {
                if global.cur_col > global.line_numbers - 1 {
                    global.cur_col -= 1;
                    let cur_col = global.cur_col;
                    global.current_line().chars.remove(cur_col as usize - 1);
                }
            },
            Command::DeleteCharX => {
                if (global.cur_col as usize - 1) < global.current_line().chars.len() {
                    let cur_col = global.cur_col;
                    global.current_line().chars.remove(cur_col as usize - 1);
                }
            },
            Command::Invalid => {},
        }
        
    }
}
