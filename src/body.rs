use error_stack::{IntoReport, Report, Result, ResultExt};
use std::{error::Error, fmt};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::commands::Commnads;
use crate::error::StatefulListStateInfoError;
#[derive(Debug)]
pub struct Body {
    pub body_mode: BodyMode,
    pub list_stateful: StatefulList<String>,
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
    fn unselect(&mut self) {
        self.state.select(None);
    }
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
        }
    }
}

impl Body {
    pub fn build_body<'a>(&mut self) -> List<'a> {
        let command1 = Commnads::new("ls".into(), "blablabl".into());
        let command2 = Commnads::new("pwd".into(), "blablasdc".into());

        let commnads = vec![command1, command2];

        let body = match self.body_mode {
            BodyMode::List => self.build_list_content(),
            BodyMode::Command => panic!("ainda não implementado"),
        };
        return body;
    }

    pub fn build_list_content<'a>(&mut self) -> List<'a> {
        let rows_list = vec!["Linux", "Todo", "logbook"];

        let item_style = Style::default().fg(Color::White).bg(Color::Black);

        let list_items: Vec<_> = rows_list
            .iter()
            .map(|item| {
                let lines = vec![Spans::from(*item)];

                ListItem::new(lines).style(item_style)
            })
            .collect();

        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("-> ");

        return list;
    }
}
