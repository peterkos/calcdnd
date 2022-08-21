

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
    pub hit_points: u8,
    pub total_hit_points: u8,
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
        saving_throws_table.set_titles(row!["Saving Throws"]);
        for saving_throw in &self.saving_throws {
            if saving_throw.valid {
                let throw_name = saving_throw.stat.to_string();
                saving_throws_table.add_row(row![throw_name]);
            }
        }

        // Hit points table
        let mut hit_points_table = Table::new();
        hit_points_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        hit_points_table.set_titles(row!["Hit Points"]);
        hit_points_table.add_row(row![format!("{}/{}", self.hit_points, self.total_hit_points)]);

        super_table.set_titles(row!["Stats", "Hit Points"]);
        super_table.add_row(row![stat_table, hit_points_table]);
        super_table.add_row(row![saving_throws_table]);

        // Write it out
        let term = Term::buffered_stdout();
        term.write_line(&super_table.to_string()).unwrap();
    }


    /// Prints only stats and returns the number of lines printed
    pub fn print_stats(&self, term: &Term) {

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

        // Write it out
        term.write_line(&stat_table.to_string()).unwrap();
        term.flush().unwrap();
    }
}
