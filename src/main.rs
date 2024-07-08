#![allow(unused_imports)]
#![warn(missing_docs)]

//! AlgebraBalancer but for a terminal instead of UWP

use std::{array, collections::{LinkedList, VecDeque}, io::{self, stdout, Read, Write}, usize};
use crossterm::{cursor::*, event::*, execute, queue, style::*, terminal::{Clear, ClearType, DisableLineWrap}, ExecutableCommand};

struct Cursor {
    col: u16,
    row: u16,
}

struct Buffer {
    buffer: String,
    cursor: Cursor,
}

impl Buffer {
    fn get_cursor_index(&self) -> usize {
        self.buffer
            .lines()
            .take((self.cursor.row as usize) + 1)
            .map(|line| line.chars().count())
            .sum::<usize>() + (self.cursor.col as usize)
    }

    fn num_lines(&self) -> u16 {
        self.buffer
            .lines()
            .count().try_into().expect("Why in god's name do you need more than 65535 lines")
    }

    fn current_line_width(&self) -> u16 {
        self.buffer
            .lines()
            .nth(self.cursor.row.into()).expect("Cursor is out of vertical bounds")
            .chars()
            .count().try_into().expect("How on earth did you write 65535 characters in one line")
    }

    /// Up
    fn move_cursor_u(&mut self, rows: u16) {
        if rows > self.cursor.row {
            self.cursor.row = 0;
            self.cursor.col = 0;
        } else {
            self.cursor.row -= rows;
        }
    }

    /// Down
    fn move_cursor_d(&mut self, rows: u16) -> bool {
        if rows > (self.num_lines() - self.cursor.row) {
            self.cursor.row = 0;
            self.cursor.col = self.current_line_width();
            false
        } else {
            self.cursor.row += rows;
            true
        }
    }

    /// Left
    fn move_cursor_l(&mut self, cols: u16) {
        let mut cols_to_move = cols;
        while cols_to_move > 0 {
            if cols_to_move > self.cursor.col {
                if self.cursor.row == 0 {
                    self.cursor.col = 0;
                    return;
                } else {
                    self.cursor.row -= 1;
                    self.cursor.col = self.current_line_width();
                    cols_to_move -= self.cursor.col;
                }
            } else {
                self.cursor.col -= cols_to_move;
                cols_to_move = 0;
            }
        }
    }

    /// Right
    fn move_cursor_r(&mut self, cols: u16) {
        let mut cols_to_move = cols;
        while cols_to_move > 0 {
            let cols_to_right = self.current_line_width() - self.cursor.col;
            if cols_to_move > cols_to_right {
                if self.cursor.row == self.num_lines() - 1 {
                    self.cursor.col = self.current_line_width();
                    return;
                } else {
                    self.cursor.row += 1;
                    self.cursor.col = 0;
                    cols_to_move -= cols_to_right;
                }
            } else {
                self.cursor.col += cols_to_move;
                cols_to_move = 0;
            }
        }
    }
}

/// The function wants to do something that needs to be done in main() (like close the program or change modes).
enum ElevatedRequest {
    EndProgram,
}

enum HandledKeypress {
    /// Should not consume the combo list.
    NoAction,

    /// Should consume the combo list.
    Action,

    /// Should consume the combo list. Requires further action.
    Elevated(ElevatedRequest),
}

fn handle_keypress_events(combo: &Vec<(KeyCode, KeyModifiers)>, buffer: &mut Buffer) -> io::Result<HandledKeypress> {
    // println!("{:?}", &combo.last().unwrap()); // Debug

    use {HandledKeypress::*, ElevatedRequest::*, KeyCode::*};
    match combo.as_slice() {
        [..,
            (Esc, KeyModifiers::NONE),
            (Esc, KeyModifiers::NONE)
        ] => {
            let mut stdout = stdout();
            queue!(
                stdout,
                SetForegroundColor(Color::Red),
                SetAttribute(Attribute::Bold),
                Print("Quitting")
            )?;
            stdout.flush()?;
            Ok(Elevated(EndProgram))
        },

        [..,
            (direction @ (Up|Down|Left|Right), KeyModifiers::NONE),
        ] => {
            let mut stdout = stdout();
            match direction {
                Up    => { buffer.move_cursor_u(1); queue!(stdout, MoveUp   (1))?; },
                Down  => { buffer.move_cursor_d(1); queue!(stdout, MoveDown (1))?; },
                Left  => { buffer.move_cursor_l(1); queue!(stdout, MoveLeft (1))?; },
                Right => { buffer.move_cursor_r(1); queue!(stdout, MoveRight(1))?; },
                _ => unreachable!(),
            };
            stdout.flush()?;
            Ok(Action)
        },

        [..,
            (Char(ch), KeyModifiers::NONE)
        ] => {
            execute!(stdout(), Print(ch))?;
            Ok(Action)
        },

        [..,
            (Backspace, KeyModifiers::NONE)
        ] => {
            let mut stdout = stdout();
            stdout.flush()?;
            Ok(Action)
        },

        [..,
            (Enter, KeyModifiers::NONE)
        ] => {
            execute!(
                stdout(),
                Print("\n"),
            )?;
            Ok(Action)
        },

        _ => Ok(NoAction)
    }
}

fn main() -> io::Result<()> {
    // Print title
    {
        let mut stdout = stdout();
        queue!(
            stdout,
            Clear(ClearType::All),
            DisableLineWrap,
            SetForegroundColor(Color::Blue),
            SetAttribute(Attribute::Bold),
            Print("AlgebraBalancer\n"),
            ResetColor,
        )?;
        stdout.flush();
    }

    let mut buffer = Buffer{ buffer: String::new(), cursor: Cursor{ col: 0, row: 0 } };
    let mut key_combo = Vec::<(KeyCode, KeyModifiers)>::new();

    // Main program loop
    loop {
        match read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                key_combo.push((event.code, event.modifiers));
                use {HandledKeypress::*, ElevatedRequest::*};
                match handle_keypress_events(&key_combo, &mut buffer)? {
                    Elevated(EndProgram) => return Ok(()),
                    Action => key_combo.clear(),
                    NoAction => (),
                }
            },
            _ => (),
        }
    }
}
