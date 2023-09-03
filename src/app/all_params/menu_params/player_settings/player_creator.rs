use crate::app::all_params::menu_params::player_settings::PlayerSettings;
use crate::app::all_params::game_params::player::Player;

pub struct PlayerCreator {
    pub func: fn(&PlayerSettings) -> Player,
    pub bot_type_id: i32,
}

impl PlayerCreator {
    pub fn new(func: fn(&PlayerSettings) -> Player, bot_type_id: i32) -> PlayerCreator {
        PlayerCreator {
            func,
            bot_type_id,
        }
    }
}

impl Default for PlayerCreator {
    fn default() -> Self {
        PlayerCreator {
            func: |_| Player::Human,
            bot_type_id: 0,
        }
    }
}

impl PartialEq for PlayerCreator {
    fn eq(&self, other: &Self) -> bool {
        self.bot_type_id == other.bot_type_id
    }
}

impl Eq for PlayerCreator {}