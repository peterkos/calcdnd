

// Make life earlier
#![allow(dead_code)]
#![feature(derive_default_enum)]
#![allow(unused_imports)]
#![allow(unreachable_code)]
#[macro_use] extern crate prettytable;

// CLI
use dialoguer::Select;
use console::Term;

// Data
mod config;
mod character;
mod data;
mod game;

use config::*;
use game::*;

use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Terminal,
};

mod event;
use event::{Event, Events};



fn main() -> Result<(), Box<dyn Error>> {

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
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
    }


    Ok(())
}
