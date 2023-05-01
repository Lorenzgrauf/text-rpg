use std::fmt::format;
use std::io;
use crate::characters::Characters::{Mage, Rouge, Warrior};
use crate::characters::CharacterSelection;

pub mod characters;

fn main() {
    let app = App {};

    app.run();
}

struct App {}

impl App {
    fn run(&self) {
        println!("You started a new game! :)");
        let character_selection = CharacterSelection {
            character_factory: || {
                loop {
                    println!("Type a number to select your character:");
                    println!("1: Rouge");
                    println!("2: Warrior");
                    println!("3: Mage");

                    let mut character_selection = String::new();

                    io::stdin()
                        .read_line(&mut character_selection)
                        .expect("You provided invalid input!");

                    let character_index: u8 = match character_selection.trim().parse() {
                        Ok(character) => character,
                        Err(_) => continue
                    };

                    return match character_index {
                        1 => Rouge,
                        2 => Warrior,
                        3 => Mage,
                        _ => continue
                    };
                }
            }
        };

        let character = character_selection.select_character();
        println!("You chose {:?}", character)
    }
}