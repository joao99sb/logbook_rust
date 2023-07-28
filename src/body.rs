use ansi_term;
use crossterm::cursor;
use error_stack::{IntoReport, Report, Result, ResultExt};
use std::io::{BufRead, BufReader, Read};
use std::vec;
use std::{fs, path::PathBuf};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::app::App;
use crate::commands::Commnads;
use crate::error::{Result as DefaultResult, StatefulListStateInfoError};

#[derive(Debug)]
pub struct Body {
    pub body_mode: BodyMode,
    pub list_stateful: StatefulList<String>,
    pub command_list: Vec<Vec<String>>,
}
#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn new() -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn next(&mut self) -> Result<(), StatefulListStateInfoError> {
        let i = match self.state.selected() {
            Some(i) => {
                let len = self.items.len();

                let check_result = match self.items.len().checked_sub(1) {
                    Some(number) => number,
                    None => {
                        return Err(Report::new(StatefulListStateInfoError).attach_printable(
                            format!("Incorrect number of items. Items length: {len} "),
                        ))
                    }
                };

                if i >= check_result {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        Ok(())
    }

    pub fn previous(&mut self) -> Result<(), StatefulListStateInfoError> {
        let i = match self.state.selected() {
            Some(i) => {
                let len = self.items.len();

                let check_result = match self.items.len().checked_sub(1) {
                    Some(number) => number,
                    None => {
                        return Err(Report::new(StatefulListStateInfoError).attach_printable(
                            format!("Incorrect number of items. Items length: {len} "),
                        ))
                    }
                };

                if i == 0 {
                    check_result
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        Ok(())
    }
    // fn unselect(&mut self) {
    //     self.state.select(None);
    // }
}
#[derive(Debug)]
pub enum BodyMode {
    Command,
    List,
}

impl Body {
    pub fn new() -> Body {
        Body {
            body_mode: BodyMode::List,
            list_stateful: StatefulList::new(),
            command_list: Vec::new(),
        }
    }
}

impl Body {
    pub fn set_body_mode(&mut self, mode: BodyMode) -> () {
        self.body_mode = mode;
    }
    pub fn set_or_unset_help_mode(&mut self) -> () {
        match self.body_mode {
            BodyMode::Command => {
                self.set_body_mode(BodyMode::List);
            }
            BodyMode::List => {
                self.set_body_mode(BodyMode::Command);
            }
        }
    }
}

impl Body {
    pub fn build_body<'a>(&mut self, commands_file: &PathBuf, current_dir: &PathBuf) -> List<'a> {
        let current_dir_str = current_dir.as_os_str().to_str().unwrap();

        let body = match self.body_mode {
            BodyMode::List => self.build_list_content(current_dir_str),
            BodyMode::Command => self.build_command_list(commands_file).unwrap(),
        };
        return body;
    }

    pub fn build_list_content<'a>(&mut self, current_dir_str: &str) -> List<'a> {
        let current_dir_vector: Vec<String> =
            current_dir_str.split('/').map(|s| s.to_string()).collect();
        let current_dir_name = current_dir_vector.last().unwrap().to_owned();

        let rows_list = Body::list_all_itens_current_dir(current_dir_str);

        // let rows_list = vec!["Linux", "Todo", "logbook"];

        let item_style = Style::default().fg(Color::White).bg(Color::Black);

        let list_items: Vec<_> = rows_list
            .iter()
            .map(|item| {
                let lines = vec![Spans::from(item.to_owned())];

                ListItem::new(lines).style(item_style)
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(current_dir_name),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        return list;
    }

    pub fn build_command_list<'a>(&mut self, commands_file: &PathBuf) -> DefaultResult<List<'a>> {
        // verification so that the configuration file doesn't need to be read every iteration.
        if self.command_list.len() == 0 {
            let file = match fs::File::open(commands_file) {
                Ok(file) => file,
                Err(e) => panic!("Erro ao tentar ler a lista de commandos: {}", e),
            };

            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.unwrap();

                let command_vector: Vec<String> = line.split('-').map(|s| s.to_string()).collect();
                self.command_list.push(command_vector);
            }
        }

        let commands: Vec<ListItem> = self
            .command_list
            .iter()
            .enumerate()
            .map(|m| {
                let command_vector = m.1;
                let command_name = (*command_vector[0]).to_string();
                let command_description = (*command_vector[1]).to_string();

                let content = vec![
                    Span::styled(command_name, Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!(": {}", command_description)),
                ];

                ListItem::new(Spans::from(content))
            })
            .collect();
        let commands_body =
            List::new(commands).block(Block::default().borders(Borders::ALL).title("Commands"));
        return Ok(commands_body);
    }
}

impl Body {
    fn list_all_itens_current_dir(current_dir: &str) -> Vec<String> {
        let entries = match fs::read_dir(current_dir) {
            Ok(entries) => entries,
            Err(e) => panic!("Erro ao ler o diretório: {}", e),
        };

        let mut all_files: Vec<String> = Vec::new();
        for entry in entries {
            let entry_it = entry.unwrap();

            let file_name = match entry_it.file_name().to_owned().to_str() {
                Some(s) => s.to_owned(),
                None => "".to_owned(),
            };

            all_files.push(file_name);
        }

        return all_files;
    }

    fn list_all_nodes(current_dir: &str) -> Vec<String> {
        let entries = match fs::read_dir(current_dir) {
            Ok(entries) => entries,
            Err(e) => panic!("Erro ao ler o diretório: {}", e),
        };

        let mut all_nodes: Vec<String> = Vec::new();
        for entry in entries {
            let entry_it = entry.unwrap();
            let entry_file_type = entry_it.file_type().unwrap();

            if entry_file_type.is_dir() {
                let file_name = match entry_it.file_name().to_owned().to_str() {
                    Some(s) => s.to_owned(),
                    None => "".to_owned(),
                };
                all_nodes.push(file_name);
            }
        }

        return all_nodes;
    }
}
