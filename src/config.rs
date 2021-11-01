

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

    pub fn create_character(&mut self) {

        // Get class
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

        // TODO
        // Get weapons
        // Get stats
        // Get saving throws
        // Get skills
    }

    pub fn pick_character(&self) {
        // TODO: Serialize to disk somehow
    }

}