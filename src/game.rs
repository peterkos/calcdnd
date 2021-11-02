

use dialoguer::{Select, Input};
use console::style;

use crate::character::*;

pub struct Game {
    pub character: Character,
}

impl Game {

    /// The main event loop for doing things!
    pub fn event_loop(&mut self) {
        let options = vec![
            "[Character]",
            "  View",
            "  Edit (unimplemented)",
            "[Action]",
            "  Attack Action",
            "  Record Damage",
            "Quit",
        ];

        loop {
            let selection = Select::new()
                .items(&options)
                .default(0)
                .interact()
                .unwrap();

            match selection {
                0 => continue,               // [Character]
                1 => self.character.print(), //   View
                2 => continue,               //   TODO: Edit
                3 => continue,               // [Action]
                4 => self.attack(),          //   Attack
                5 => self.damage(),          //   Record Damage
                6 => return,                 //  Quit
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

        println!("Did {} damage.", style(total).yellow());
    }

    fn damage(&mut self) {

        let damage: u8 = Input::new()
            .with_prompt("How much damage?")
            .interact_text()
            .unwrap();

        let calc = self.character.hit_points as i8 - damage as i8;

        println!("  Old HP: {}/{}", self.character.hit_points, self.character.total_hit_points);
        println!("  New HP: {}/{}", calc, self.character.total_hit_points);

        // Death!
        if calc <= 0 {
            // TODO: Handle Death.
            // Maybe print last character sheet w/ red skull at top?
            println!("{}", style("-------------").magenta());
            println!("{}{}{}", style("|").magenta(), style(" YOU DIED. ").red(), style("|").magenta());
            println!("{}", style("-------------").magenta());
        } else {
            self.character.hit_points -= calc as u8;
        }
    }
}
