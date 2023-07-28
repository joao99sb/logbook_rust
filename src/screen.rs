
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders,  Paragraph, },
    Frame, Terminal,
};

use crate::{body::BodyMode, App, InputMode};
pub struct Screen {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Screen {
    pub fn new() -> Result<Screen, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
        let terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

        Ok(Screen { terminal })
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

    let help_message = build_header(app);
    f.render_widget(help_message, chunks[0]);

    let commands_file = app.command_file.clone();
    let current_dir = app.current_dir.clone();
     
    let body = app.body.build_body(&commands_file, &current_dir);

    match app.body.body_mode {
        BodyMode::List => {
            f.render_stateful_widget(body, chunks[1], &mut app.body.list_stateful.state)
        }
        BodyMode::Command => f.render_widget(body, chunks[1]),
    }

    let input = build_input(app);
    f.render_widget(input, chunks[2]);
}

fn build_header(app: &App) -> Paragraph<'_> {
    let (msg, style) = match app.get_input_mode() {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to type, "),
                Span::styled("h", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to list commands"),
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
    return help_message;
}

fn build_input(app: &App) -> Paragraph<'_> {
    let input_style = match app.get_input_mode() {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Yellow),
    };

    let input_block_style = Block::default().borders(Borders::ALL).title("Input");

    let input = Paragraph::new(app.input.as_ref())
        .style(input_style)
        .block(input_block_style);
    return input;
}
