

// Make life earlier
#![allow(dead_code)]

// CLI
use dialoguer::Select;
use console::Term;

// Data
mod character;
use character::*;

use strum::VariantNames;


fn main() -> std::io::Result<()> {


    let selections = Select::new()
        .item("New Character")
        .item("Select existing character")
        .interact_on(&Term::stderr())?;

    match selections {
        0 => new_character(),
        1 => pick_character(),
        _ => unimplemented!()
    }

    Ok(())
}

fn new_character() {

    let classes = Class::VARIANTS;
    let selections = Select::new()
        .items(&classes)
        .default(0)
        .interact_on(&Term::stderr());
}

fn pick_character() {
    // TODO: Also serialize to disk (serde?)
}
