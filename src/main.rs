

// Make life earlier
#![allow(dead_code)]

// CLI
use dialoguer::Select;
use console::Term;

// Data
mod character;
use character::*;

// Other
use strum::VariantNames;
use std::str::FromStr;


fn main() -> std::io::Result<()> {

    let selections = Select::new()
        .default(0)
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
    let select_index = Select::new()
        .items(&classes)
        .default(0)
        .interact()
        .unwrap();

    let class = Class::from_str(classes[select_index]).unwrap();

    match class {
        Class::Barbarian => println!("Yay!")
    }


}

fn pick_character() {
    // TODO: Also serialize to disk (serde?)
}
