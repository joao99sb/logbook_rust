use std::io::{self, Write};
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::body::{Body, BodyMode};
use crate::commands::Commnads;
use crate::keyEnvents::listen_key_events;
use crate::screen::ui;

#[derive(Debug)]
pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub input_content: Vec<String>,
    pub body: Body,

    pub commands: Vec<Commnads>,
    pub meta_file: PathBuf,
    pub current_dir: PathBuf,
    pub command_file: PathBuf,
    pub is_running: bool,
}
#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            commands: Vec::new(),
            input_content: Vec::new(),
            body: Body::new(),
            meta_file: PathBuf::new(),
            is_running: true,
            current_dir: PathBuf::new(),
            command_file: PathBuf::new(),
        }
    }
}

impl App {
    pub fn run_app<B: Backend>(mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui(f, &mut self))?;

            listen_key_events(&mut self)?;
            if !self.is_running {
                return Ok(());
            }
        }
    }
}

impl App {
    pub fn make_metadata_file(&mut self) -> io::Result<()> {
        let root_dir = std::env::current_dir()?; // /home/joao99sb/code/logbook
        let file_path = root_dir.join(".metadadata");
        let root_path = file_path.join("root");
        if !file_path.exists() {
            std::fs::create_dir(&file_path)?;
            std::fs::create_dir(&root_path)?;
            self.config_commands(&file_path)?;
        }

        let config_file_path = file_path.join("commands.txt");

        self.meta_file = file_path;
        self.current_dir = root_path;

        self.command_file = config_file_path;

        Ok(())
    }

    fn config_commands(&mut self, file_path: &PathBuf) -> io::Result<()> {
        let config_file_path = file_path.join("commands.txt");

        let mut config_file = std::fs::File::create(config_file_path)?;
        let default_commands = vec![
            "mkdir <Node Name>-Create new Node",
            "rm <Node Name>-Remove empty Nodes",
        ];

        for comando in default_commands {
            writeln!(config_file, "{}", comando)?;
        }
        Ok(())
    }
}

impl App {
    pub fn close_app(&mut self) -> () {
        self.is_running = false;
    }
}
