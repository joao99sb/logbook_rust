mod app;
mod body;
mod commands;
mod error;
mod event;
mod screen;

use std::io;

use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

use crate::app::{App, InputMode};
use crate::error::Result;
use crate::screen::Screen;

fn main() -> Result<()> {
    // setup terminal

    let mut screen: Screen = match Screen::new() {
        Ok(screen) => screen,
        Err(e) => panic!("deu erro {}", e),
    };

    // error handling
    chain_hook();

    let mut app = App::default();

    match app.make_metadata_file() {
        Ok(()) => {}
        Err(e) => panic!("deu erro ao tentar criar o arquivo {}", e),
    }

    //provisório
    let rows_list = vec!["Linux", "Todo", "logbook"];
    for var in rows_list.iter() {
        app.body.list_stateful.add_item(var.to_string());
    }
    let res = app.run_app(&mut screen.terminal);

    // restore terminal
    screen.destroy()?;

    if let Err(err) = res {
        println!("erro: {:?}", err)
    }

    Ok(())
}

fn chain_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));
}

fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
