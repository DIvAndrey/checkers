use crate::bot::Bot;

pub enum Player {
    Human,
    Computer(Box<dyn Bot>),
}

impl Player {
    pub fn recreate_bot(&mut self) {
        match self {
            Player::Human => {}
            Player::Computer(bot) => bot.recreate(),
        }
    }

    pub fn get_computer_mut(&mut self) -> &mut Box<dyn Bot> {
        match self {
            Player::Human => panic!("Should be Computer"),
            Player::Computer(computer) => computer
        }
    }
}

impl Default for Player {
    #[inline(always)]
    fn default() -> Self {
        Player::Human
    }
}
