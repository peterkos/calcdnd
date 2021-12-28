

// Make life earlier
#![allow(dead_code)]
#![feature(derive_default_enum)]

#[macro_use] extern crate prettytable;

// CLI
use dialoguer::Select;
use console::Term;

// Data
mod config;
mod character;
mod data;
mod game;
mod dragon;
use config::*;
use game::*;

pub fn main() -> std::io::Result<()> {
    dragon::loading();

    let selections = Select::new()
        .default(0)
        .item("New Character")
        .item("Select existing character")
        .item("Quit")
        .interact_on(&Term::stderr())?;

    let mut config = Config::default();

    match selections {
        0 => config.create_character(),
        1 => config.pick_character(),
        2 => return Ok(()),
        _ => unimplemented!()
    }

    let mut game = Game{ character: config.character };

    game.event_loop();

    Ok(())
}
