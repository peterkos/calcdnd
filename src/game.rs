

use dialoguer::{Select, Input};
use console::Style;

use crate::character::*;

pub struct Game {
    pub character: Character,
}

impl Game {

    /// The main event loop for doing things!
    pub fn event_loop(&self) {
        let options = vec![
            "[Action]",
            "  Attack Action",
            "  Record Damage",
            "  Quit",
            "[Character]",
            "  View",
            "  Edit (unimplemented)",
        ];

        loop {
            let selection = Select::new()
                .items(&options)
                .default(0)
                .interact()
                .unwrap();

            match selection {
                0 => continue, // [Action]
                1 => self.attack(),
                2 => continue, // TODO: self.damage()
                3 => return,       // quit
                4 => continue, // [Character]
                5 => self.character.print(),
                6 => continue, // TODO: Edit
                _ => continue
            };
        }

    }

    fn attack(&self) {

        println!("Attacking!");

        // Pick weapon
        let weapons = &self.character.weapons;
        if weapons.is_empty() {
            println!("Error: No weapons specified.");
            return;
        }

        // TODO: Spells vs. weapons

        let selection = Select::new()
            .with_prompt("Which weapon?")
            .items(&weapons)
            .default(0)
            .interact();

        let weapon = match selection {
            Ok(s)  => &weapons[s],
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };

        // TODO: Finesse
        // TODO: Melee vs. Ranged
        //      melee - Strength
        //      ranged - Dexterity

        // Assuming melee for now
        let dmg_roll: u8 = Input::new()
            .with_prompt(format!("Roll {}d{}", weapon.dice_count, weapon.dice))
            .interact_text()
            .unwrap();

        let mut total = dmg_roll;

        // TODO: Check if proficient per-class, func on class?
        total += self.character.proficiency_bonus;
        total += self.character.stats.strength;

        let cyan = Style::new().cyan();
        println!("Did {} damage.", cyan.apply_to(total));


    }

}