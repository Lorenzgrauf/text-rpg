use crate::character_selection::CharacterSelection;
use crate::game::Game;
use crate::view::Menu;

pub mod characters;
mod character_selection;
mod game;
mod view;

fn main() {
    let app = App;

    app.run();
}

struct App;

impl App {
    fn run(&self) {
        println!("You started a new game! :)");
        let character_selection = CharacterSelection::stdin_character_factory();

        let character = character_selection.character();
        println!("You chose {:?}", character);

        let mut game = Game::new(character, Box::new(Menu {
            view_factory: Box::new(|| todo!()),
        }));

        game.play();
    }
}