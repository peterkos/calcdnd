

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


    let ui = UI{ state: ScreenState::New };
    match ui.start() {
        Ok(()) => (),
        Err(e) => panic!("Error starting UI.")
    };


}
