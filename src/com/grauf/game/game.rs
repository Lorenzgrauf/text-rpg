use crate::characters::Character;
use crate::view::{Menu, View};

pub struct Game {
    character: Character,
    view: Box<dyn View>,
}

impl Game {
    pub(crate) fn play(&mut self) {
        let next_view = self.view.next_view();
        self.view = next_view
    }
}

impl Game {
    pub(crate) fn new(character: Character, view: Box<dyn View>) -> Game {
        Game {
            character,
            view
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::characters::Character::Rouge;
    use crate::game::Game;
    use crate::view::Menu;

    #[test]
    fn play_should_not_throw_an_exception() {
        let mut game = Game::new(Rouge, Box::new(Menu {
            view_factory: Box::new(|| panic!("Do not call me!"))
        }));

        game.play();
    }
}