

use dialoguer::{Select, Input};
use console::Style;

use crate::character::*;

pub struct Game {
    pub character: Character,
}

impl Game {

    /// The main event loop for doing things!
    pub fn event_loop(&self) {
        loop {
            let options = vec![
                "Attack Action",
                "Record Damage",
                "Quit"
            ];

            let selection = Select::new()
                .items(&options)
                .default(0)
                .interact()
                .unwrap();

            match selection {
                0 => self.attack(),
                2 => return,
                _ => ()
            };
        }

    }

    fn attack(&self) {

        // Pick weapon
        let weapons = &self.character.weapons;
        if weapons.is_empty() {
            println!("Error: No weapons specified.");
            return;
        }
        let selection = Select::new()
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