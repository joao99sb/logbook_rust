use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    Frame, Terminal,
};

use crate::{body::BodyMode, App, InputMode};
pub struct Screen {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
    int: String,
}

impl Screen {
    pub fn new() -> Result<Screen, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
        let terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

        Ok(Screen {
            terminal,
            int: String::from("olas"),
        })
    }

    pub fn destroy(mut self) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Percentage(80),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to type"),
            ],
            Style::default().add_modifier(Modifier::REVERSED),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);

    f.render_widget(help_message, chunks[0]);

    let body = app.body.build_body();

    match app.body.body_mode {
        BodyMode::List => {
            f.render_stateful_widget(body, chunks[1], &mut app.body.list_stateful.state)
        }
        BodyMode::Command => panic!("implementar ainda"),
    }

    let input_style = match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Yellow),
    };

    let input_block_style = Block::default().borders(Borders::ALL).title("Input");

    let input = Paragraph::new(app.input.as_ref())
        .style(input_style)
        .block(input_block_style);

    f.render_widget(input, chunks[2]);
}
