pub struct CharacterSelection {
    pub(crate) character_factory: fn() -> Characters,
}

impl CharacterSelection {

    pub fn select_character(&self) -> Characters {
        (self.character_factory)()
    }
}

#[derive(Debug, PartialEq)]
pub enum Characters {
    Rouge,
    Warrior,
    Mage
}


#[cfg(test)]
mod tests {
    use crate::characters::Characters::Rouge;
    use crate::characters::CharacterSelection;

    #[test]
    fn spike() {
        let character_selection = CharacterSelection {
            character_factory: || Rouge
        };

        let character = character_selection.select_character();

        assert_eq!(character, Rouge)
    }
}