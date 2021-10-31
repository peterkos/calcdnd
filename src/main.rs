

// Make life earlier
#![allow(dead_code)]

use dialoguer::Select;
use console::Term;

fn main() -> std::io::Result<()> {


    let mut characters: Vec<Character> = Vec::new();

    let selections = Select::new()
        .item("New Character")
        .item("Select existing character")
        .interact_on(&Term::stderr())?;

    match selections {
        0 => unimplemented!(),
        1 => {

        },
        _ => unimplemented!()
    }

    Ok(())
}


struct Character {
    stats: Stats,
    saving_throws: SavingThrows,
    skills: Skills
}

struct Stats {
    strength: String,
    dexterity: String,
    constitution: String,
    intelligence: String,
    wisdom: String,
    charisma: String
}

enum Stat {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

struct SavingThrows {
    strength:     (bool, u16),
    dexterity:    (bool, u16),
    constitution: (bool, u16),
    intelligence: (bool, u16),
    wisdom:       (bool, u16),
    charisma:     (bool, u16),
}

struct Skills {
    // TODO: Fill in
}

/// Supported classes
enum Class {
    Barbarian
    // TODO: Do more :)
}


enum Weapon {
    Club(Dice, Vec<Properties>),
    Dagger(Dice, Vec<Properties>),
    Greatclub(Dice, Vec<Properties>)
    // TODO: More weapons
}

enum Properties {
    // TODO
}

enum Dice {
    D3,
    D4,
    D5,
    D6,
    D8,
    D10,
    D12
}