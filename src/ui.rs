

use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{Terminal,
          backend::TermionBackend,
          layout::{Alignment, Constraint, Direction, Layout},
          style::{Color, Modifier, Style},
          text::{Span, Spans},
          widgets::{Widget, Block, BorderType, Borders, Row, Table, TableState, Cell}};



use crate::event::{Event, Events};
use crate::character::*;


// MARK: - Datatypes

pub enum ScreenState {
    New,
    ViewCharacter,
    // EditCharacter
}


// MARK: - UI

pub struct UI {
    pub state: ScreenState,
    pub character: Character,
    table_state: TableState
}


impl UI {

    pub fn new(state: ScreenState, character: Character) -> UI {
        UI { state, character, table_state: TableState::default() }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {

        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let events = Events::new();

        Ok(loop {
            terminal.draw(|f| {

                let size = f.size();

                let block = Block::default()
                    .borders(Borders::ALL)
                    .title("Test Title")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded);
                // f.render_widget(block, size);

                match self.state {
                    ScreenState::New           => f.render_widget(UI::draw_config(), size),
                    ScreenState::ViewCharacter => f.render_stateful_widget(UI::draw_character(), size, &mut self.table_state)
                };
            })?;

            // Match events
            if let Event::Input(key) = events.next()? {
                match key {
                    Key::Char('q') => break,
                    Key::Char('1') => self.state = ScreenState::New,
                    Key::Char('2') => self.state = ScreenState::ViewCharacter,
                    Key::Down | Key::Right => self.next(),
                    Key::Up   | Key::Left  => self.prev(),
                    _ => continue
                }
            }
        })
    }

    fn draw_config() -> Block<'static> {
        let block = Block::default()
                    .borders(Borders::ALL)
                    .title("Config")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded);

        block
    }

    fn draw_character() -> Table<'static> {

        // stat_table.set_titles(row!["Stat", "Value"]);
        // stat_table.add_row(row!["Strength",     self.stats.strength]);
        // stat_table.add_row(row!["Dexterity",    self.stats.dexterity]);
        // stat_table.add_row(row!["Constitution", self.stats.constitution]);
        // stat_table.add_row(row!["Intelligence", self.stats.intelligence]);
        // stat_table.add_row(row!["Wisdom",       self.stats.wisdom]);
        // stat_table.add_row(row!["Charisma",     self.stats.charisma]);




        // let rows = Row::new(vec!["First", "Second", "Third"]);

        // let table = Table::new(vec![rows])
        //     .style(Style::default().fg(Color::White));
        // table
        //

        let table = Table::new(vec![
            Row::new(vec!["Row11", "Row12", "Row13"]),
            Row::new(vec!["Row11", "Row12", "Row13"]),
            Row::new(vec!["Row11", "Row12", "Row13"])])
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
            .style(Style::default().fg(Color::Yellow))
        .bottom_margin(1)
        )
        .block(Block::default().title("Table"))
        .widths(&[
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(10)])
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

        table

    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some (i) => {
                if i > 2 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0
        };
        self.table_state.select(Some(i));
    }

    fn prev(&mut self) {
        let i = match self.table_state.selected() {
            Some (i) => {
                if i == 0 {
                    2
                } else {
                    i - 1
                }
            }
            None => 0
        };
        self.table_state.select(Some(i));
    }
}
