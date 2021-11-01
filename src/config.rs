

use dialoguer::Select;

// Data
use crate::character::*;
use crate::data::*;

// Other
use strum::VariantNames;
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub character: Character
}

impl Default for Config {
    fn default() -> Config {
        Config{character: Character::default()}
    }
}

impl Config {

    // MARK: - Main functions

    /// Create a new character
    pub fn create_character(&mut self) {
        self.class();
        self.stats();
        self.weapons();
        self.saving_throws();
        self.skills();
    }

    /// Pick a character
    pub fn pick_character(&self) {
        // TODO: Serialize to disk somehow
        unimplemented!()
    }

    // MARK: Supporting functions

    fn class(&mut self) {
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

        self.character.class = class;
    }

    fn stats(&self) {
        unimplemented!()
    }
    fn weapons(&self) {
        unimplemented!()
    }
    fn saving_throws(&self) {
        unimplemented!()
    }
    fn skills(&self) {
        unimplemented!()
    }
}
