use std::io;

use crossterm::event::{self, Event, KeyCode};

use crate::error::Result;
use crate::{
    app::{App, InputMode},
    body::BodyMode,
};

pub fn listen_key_events(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    app.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    app.close_app();
                }
                KeyCode::Char('h') => app.body.set_or_unset_help_mode(),
                KeyCode::Up => app.body.list_stateful.previous().unwrap(),
                KeyCode::Down => app.body.list_stateful.next().unwrap(),
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    app.input_content.push(app.input.drain(..).collect());
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            },
        }
    }
    Ok(())
}
