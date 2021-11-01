

use crate::data::*;


#[derive(Default, Debug)]
pub struct Character {
    pub class: Class,
    pub stats: Stats,
    pub weapons: Vec<Weapon>,
    // pub saving_throws: SavingThrows,
    // pub skills: Skills
}
