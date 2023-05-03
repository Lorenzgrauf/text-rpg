use crate::characters::{CharacterSelection};

pub mod characters;

fn main() {
    let app = App {};

    app.run();
}

struct App {}

impl App {
    fn run(&self) {
        println!("You started a new game! :)");
        let character_selection = CharacterSelection::stdin_character_factory();

        let character = character_selection.character();
        println!("You chose {:?}", character)
    }
}