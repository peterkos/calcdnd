

use crate::data::*;
use serde::Serialize;
use serde::Deserialize;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Character {
    pub class: Class,
    pub stats: Stats,
    pub weapons: Vec<Weapon>,
    // pub saving_throws: SavingThrows,
    // pub skills: Skills
}
