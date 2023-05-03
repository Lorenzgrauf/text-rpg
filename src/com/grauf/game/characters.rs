use std::io;
use crate::characters::Character::{Mage, Rouge, Warrior};

pub struct CharacterSelection {
    pub character_factory: Box<dyn Fn() -> Character>,
}

impl CharacterSelection {
    pub fn stdin_character_factory() -> CharacterSelection {
        CharacterSelection {
            character_factory: Box::new(|| {
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
            })
        }
    }

    pub fn character(&self) -> Character {
        (self.character_factory)()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Character {
    Rouge,
    Warrior,
    Mage,
}


#[cfg(test)]
mod tests {
    use crate::characters::Character::{Mage, Rouge, Warrior};
    use crate::characters::CharacterSelection;

    #[test]
    fn select_should_provide_character() {
        for character in [Rouge, Warrior, Mage].iter() {
            let character_selection = CharacterSelection {
                character_factory: Box::new(move || character.clone())
            };

            let selected_character = &character_selection.character();

            assert_eq!(selected_character, character)
        }
    }
}