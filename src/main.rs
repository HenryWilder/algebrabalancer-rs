#![allow(unused_imports)]

use std::io::{stdout, Write};
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState},
    execute,
    style::*,
    ExecutableCommand
};

fn main() -> std::io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetAttribute(Attribute::Bold),
        Print("AlgebraBalancer\n"),
        ResetColor
    )?;

    let mut last_key_event: Option<KeyEvent> = None;

    loop {
        match read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                println!("{:?}", event);
                if let Some(last_key_event) = last_key_event {
                    match (event, last_key_event) {
                        (KeyEvent{ code: KeyCode::Esc, .. }, KeyEvent{ code: KeyCode::Esc, .. }) => {
                            execute!(
                                stdout(),
                                SetForegroundColor(Color::Red),
                                SetAttribute(Attribute::Bold),
                                Print("Quitting"),
                            )?;
                            return Ok(());
                        },
                        _ => (),
                    }
                }
                last_key_event = Some(event);
            },
            _ => (),
        }
    }
}
