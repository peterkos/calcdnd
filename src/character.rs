

use crate::data::*;
use serde::Serialize;
use serde::Deserialize;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub level: u8,
    pub proficiency_bonus: u8,
    pub stats: Stats,
    pub weapons: Vec<Weapon>,
    // pub saving_throws: SavingThrows,
    // pub skills: Skills
}

impl Character {

    /// Pre: level and class are already defined
    pub fn calc_proficiency(&mut self) {
        // TODO: Assumed equal for all classes
        match self.level {
            1..=4   => self.proficiency_bonus = 2,
            5..=8   => self.proficiency_bonus = 3,
            9..=12  => self.proficiency_bonus = 4,
            13..=16 => self.proficiency_bonus = 5,
            17..=20 => self.proficiency_bonus = 6,
            _ => ()
        };
    }
}
