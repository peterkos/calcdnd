

use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Table, BorderType, Borders},
    Terminal,
};

use crate::event::{Event, Events};



// MARK: - Datatypes

pub enum ScreenState {
    New,
    // ViewCharacter,
    // EditCharacter
}


// MARK: - UI

pub struct UI {
    pub state: ScreenState
}


impl UI {

    pub fn start(&self) -> Result<(), Box<dyn Error>> {

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
                f.render_widget(block, size);


            })?;

            if let Event::Input(key) = events.next()? {
                if key == Key::Char('q') {
                    break;
                }
            }
        })
    }
}
