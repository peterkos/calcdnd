

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
mod event;
mod ui;

use config::*;
use game::*;
use ui::*;


fn main()  {

    let mut config = Config::default();

    config.pick_character();

    let mut ui = UI::new(ScreenState::ViewCharacter, config.character);
    match ui.start() {
        Ok(()) => (),
        Err(e) => panic!("Error starting UI: {}", e)
    };


}
