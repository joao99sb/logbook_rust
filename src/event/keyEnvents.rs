use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

use crate::app::{App, InputMode};

pub fn listen_key_events(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match app.get_input_mode() {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    app.change_input_mode(InputMode::Editing);
                }
                KeyCode::Char('q') => {
                    app.close_app();
                }
                KeyCode::Char('h') => app.body.set_or_unset_help_mode(),
                KeyCode::Up => app.body.list_stateful.previous().unwrap(),
                KeyCode::Down => app.body.list_stateful.next().unwrap(),
                _ => {}
            },
            InputMode::Editing => edit_mode_fn(key, app),
        }
    }
    Ok(())
}

fn edit_mode_fn(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Enter => {
            app.input_content.push(app.input.drain(..).collect());
        }
        KeyCode::Esc => {
            app.change_input_mode(InputMode::Normal);
        }
        _ => {}
    }
}
