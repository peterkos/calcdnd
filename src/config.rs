

use console::style;
use dialoguer::{Confirm, MultiSelect, Select, Input};
use serde::Serialize;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use strum::VariantNames;
use std::str::FromStr;

// Data
use crate::character::*;
use crate::data::*;


#[derive(Debug, Serialize, Deserialize)]
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
        self.name();
        self.class();
        self.level();
        self.character.calc_proficiency();
        self.stats();
        self.hit_points();
        self.weapons();
        self.saving_throws();
        // self.skills();
        self.save_prompt();
    }

    /// Pick a character
    /// TODO: Allow for select/storage of _multiple_ characters
    pub fn pick_character(&mut self) {

        let file = match File::open("info.json") {
            Err(e) => panic!("Can't find character file \"info.json\": {}", e),
            Ok(f) => f

        };

        let buffer_in = BufReader::new(file);
        let serialized = buffer_in.lines().next().unwrap();

        // Input errors
        let input: Result<Character, serde_json::Error> = match serialized {
            Ok(s) => serde_json::from_str(&s),
            Err(e) => panic!("Unable to deserialize character: {}", e)
        };

        // Serde parsing errors
        let character = match input {
            Ok(c) => c,
            Err(e) => panic!("Unable to read character. {}", e)
        };

        self.character = character;
        println!("Character {} imported sucessfully.", style(&self.character.name).green());
        ()
    }

    // MARK: Supporting functions

    fn name(&mut self) {
        let name = Input::new().with_prompt("Name").interact_text().unwrap();
        self.character.name = name;
    }

    fn level(&mut self) {
        let level = Input::new().with_prompt("Level").interact_text().unwrap();
        self.character.level = level;
    }

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

    fn stats(&mut self) {
        let strength     = Input::new().with_prompt("Strength").interact_text().unwrap();
        let dexterity    = Input::new().with_prompt("Dexterity").interact_text().unwrap();
        let constitution = Input::new().with_prompt("Constitution").interact_text().unwrap();
        let intelligence = Input::new().with_prompt("Intelligence").interact_text().unwrap();
        let wisdom       = Input::new().with_prompt("Wisdom").interact_text().unwrap();
        let charisma     = Input::new().with_prompt("Charisma").interact_text().unwrap();

        self.character.stats = Stats { strength, dexterity, constitution, intelligence, wisdom, charisma };
    }

    fn hit_points(&mut self) {
        let total_hit_points = Input::new()
            .with_prompt("Total hit points")
            .interact_text()
            .unwrap();

        // Assumed full health at start
        self.character.hit_points = total_hit_points;
        self.character.total_hit_points = total_hit_points;
    }

    fn weapons(&mut self) {

        let weapons: Vec<Weapon> = self.build_weapons();

        let selected_weapons = MultiSelect::new()
            .items(&weapons)
            .interact()
            .unwrap();


        let mapped_weapons = selected_weapons
            .into_iter().map(|w|
                weapons[w].clone()
            )
            .collect();

        self.character.weapons = mapped_weapons;

        println!("User selected {:?}", self.character.weapons);

    }

    fn saving_throws(&mut self) {

        // First, multi-select them
        let stats = Stat::VARIANTS;
        let selected_throws = MultiSelect::new()
            .items(&stats)
            .interact()
            .unwrap();

        let mapped_throws: _ = selected_throws
            .into_iter()
            .map(|s|
                Stat::from_str(stats[s].clone()).unwrap()
            )
            .map(|s|
                SavingThrow {valid: true, stat: s}
            )
            .collect();

        self.character.saving_throws = mapped_throws;
    }

    fn skills(&self) {
        unimplemented!()
    }

    fn save_prompt(&self) {

        let save = Confirm::new()
            .with_prompt("Save your new character?")
            .interact()
            .unwrap();

        if !save {
            ()
        }

        let mut file = match File::create("info.json"){
            Err(e) => panic!("Can't make character file: {}", e),
            Ok(f) => f
        };

        let contents = serde_json::to_string(&self.character).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    // MARK: - Helper build functions

    fn build_weapons(&self) -> Vec<Weapon> {

        // TODO: Do this from a database or something

        let mut weapons: Vec<Weapon> = Vec::new();

        weapons.push(Weapon {name: String::from("Club"),      dice_count: 1, dice: Dice::D4, properties: Vec::new() });
        weapons.push(Weapon {name: String::from("Dagger"),    dice_count: 1, dice: Dice::D4, properties: Vec::new() });
        weapons.push(Weapon {name: String::from("Greatclub"), dice_count: 1, dice: Dice::D8, properties: Vec::new() });

        weapons

    }
}
