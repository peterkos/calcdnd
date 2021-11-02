

use serde::Serialize;
use serde::Deserialize;
use prettytable::{format, Table};
use console::Term;

use crate::data::*;


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub level: u8,
    pub proficiency_bonus: u8,
    pub stats: Stats,
    pub saving_throws: Vec<SavingThrow>,
    pub weapons: Vec<Weapon>,
    // pub skills: Skills
}

impl Character {

    /// Pre: level is already defined
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

    /// Outputs the character... with tables!
    pub fn print(&self) {
        let mut super_table = Table::new();
        super_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        // Stats table
        let mut stat_table = Table::new();
        stat_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        stat_table.set_titles(row!["Stat", "Value"]);
        stat_table.add_row(row!["Strength",     self.stats.strength]);
        stat_table.add_row(row!["Dexterity",    self.stats.dexterity]);
        stat_table.add_row(row!["Constitution", self.stats.constitution]);
        stat_table.add_row(row!["Intelligence", self.stats.intelligence]);
        stat_table.add_row(row!["Wisdom",       self.stats.wisdom]);
        stat_table.add_row(row!["Charisma",     self.stats.charisma]);

        // Saving throws table
        let mut saving_throws_table = Table::new();
        saving_throws_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        saving_throws_table.set_titles(row!["Y/N", "Modifier"]);
        for saving_throw in &self.saving_throws {
            let valid = saving_throw.valid;
            let stat  = saving_throw.stat.to_string();
            saving_throws_table.add_row(row![valid, stat]);
        }

        super_table.set_titles(row!["Stats", "Saving Throws"]);
        super_table.add_row(row![stat_table, saving_throws_table]);

        // Write it out
        let term = Term::stderr();
        term.write_line(&super_table.to_string()).unwrap();
    }
}
