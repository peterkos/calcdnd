

use strum_macros::{EnumString, EnumVariantNames};
use strum::VariantNames;

pub struct Character {
    stats: Stats,
    saving_throws: SavingThrows,
    skills: Skills
}

pub struct Stats {
    strength: String,
    dexterity: String,
    constitution: String,
    intelligence: String,
    wisdom: String,
    charisma: String
}

pub enum Stat {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

pub struct SavingThrows {
    strength:     (bool, u16),
    dexterity:    (bool, u16),
    constitution: (bool, u16),
    intelligence: (bool, u16),
    wisdom:       (bool, u16),
    charisma:     (bool, u16),
}

pub struct Skills {
    // TODO: Fill in
}

/// Supported classes
#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "title_case")]
pub enum Class {
    Barbarian
    // TODO: Do more :)
}


pub enum Weapon {
    Club(Dice, Vec<Properties>),
    Dagger(Dice, Vec<Properties>),
    Greatclub(Dice, Vec<Properties>)
    // TODO: More weapons
}

pub enum Properties {
    // TODO
}

pub enum Dice {
    D3,
    D4,
    D5,
    D6,
    D8,
    D10,
    D12
}