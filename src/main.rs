

// Make life earlier
#![allow(dead_code)]
#![feature(derive_default_enum)]

// CLI
use dialoguer::Select;
use console::Term;

// Data
mod config;
mod character;
mod data;

use config::*;

fn main() -> std::io::Result<()> {

    let selections = Select::new()
        .default(0)
        .item("New Character")
        .item("Select existing character")
        .interact_on(&Term::stderr())?;

    let mut config = Config::default();

    match selections {
        0 => config.create_character(),
        1 => config.pick_character(),
        _ => unimplemented!()
    }

    Ok(())
}
