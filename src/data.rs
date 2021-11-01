

/// A bunch of datatypes that will eventually be refactored
/// to their own files as implemented.
/// For now, it's easier to see them all at once here.


use strum_macros::{EnumString, EnumVariantNames};
use parse_display::{Display};

#[derive(Default, Debug)]
pub struct Stats {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8
}

impl Stats {
    /// Gets modifier from an incoming int
    pub fn modifier(num: u8) -> i8 {
        match num {
            1       => -5,
            2 | 3   => -4,
            4 | 5   => -3,
            6 | 7   => -2,
            8 | 9   => -1,
            10 | 11 => 0,
            12 | 13 => 1,
            14 | 15 => 2,
            16 | 17 => 3,
            18 | 19 => 4,
            20 | 21 => 5,
            22 | 23 => 6,
            24 | 25 => 7,
            26 | 27 => 8,
            28 | 29 => 9,
            30      => 10,
            _       => num as i8
        }
    }
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
#[derive(Default, Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "title_case")]
pub enum Class {
    #[default] Barbarian
    // TODO: Do more :)
}

#[derive(Display, Debug, Clone)]
#[display("{name}, {dice_count}d{dice}")]
pub struct Weapon {
    pub name: String,
    pub dice_count: u8,
    pub dice: Dice,
    pub properties: Vec<Properties>
}


#[derive(Debug, Clone)]
pub enum Properties {
    Empty
}

#[derive(Display, Debug, Clone)]
pub enum Dice {
    D3,
    D4,
    D5,
    D6,
    D8,
    D10,
    D12
}