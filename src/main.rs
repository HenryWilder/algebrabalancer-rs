#![allow(unused_imports)]
#![warn(missing_docs)]

//! AlgebraBalancer but for a terminal instead of UWP

use std::{array, collections::{LinkedList, VecDeque}, io::{self, stdout, Write}};
use crossterm::{cursor, event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, execute, style::*, ExecutableCommand};

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

fn handle_keypress_events(combo: &mut Vec<(KeyCode, KeyModifiers)>) -> io::Result<HandledKeypress> {
    // println!("{:?}", &combo.last().unwrap()); // Debug

    use {HandledKeypress::*, ElevatedRequest::*, KeyCode::*};
    match combo.as_slice() {
        [..,
            (Esc, KeyModifiers::NONE),
            (Esc, KeyModifiers::NONE)
        ] => {
            execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                SetAttribute(Attribute::Bold),
                Print("Quitting"),
            )?;
            Ok(Elevated(EndProgram))
        },

        [..,
            (direc @ (Up|Down|Left|Right), KeyModifiers::NONE),
        ] => {
            match direc {
                Up    => stdout().execute(cursor::MoveUp   (1))?,
                Down  => stdout().execute(cursor::MoveDown (1))?,
                Left  => stdout().execute(cursor::MoveLeft (1))?,
                Right => stdout().execute(cursor::MoveRight(1))?,
                _ => unreachable!(),
            };
            Ok(Action)
        }

        _ => Ok(NoAction)
    }
}

fn main() -> io::Result<()> {
    // Print title
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetAttribute(Attribute::Bold),
        Print("AlgebraBalancer\n"),
        ResetColor
    )?;

    // Main program loop
    let mut key_combo = Vec::<(KeyCode, KeyModifiers)>::new();
    loop {
        match read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                key_combo.push((event.code, event.modifiers));
                use {HandledKeypress::*, ElevatedRequest::*};
                match handle_keypress_events(&mut key_combo)? {
                    Elevated(EndProgram) => return Ok(()),
                    Action => (),
                    NoAction => (),
                }
            },
            _ => (),
        }
    }
}
