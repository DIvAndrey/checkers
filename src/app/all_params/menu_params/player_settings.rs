use crate::app::all_params::game_params::player::Player;
use crate::app::all_params::menu_params::player_settings::player_creator::PlayerCreator;

pub mod player_creator;

pub struct PlayerSettings {
    pub player_creator: PlayerCreator,
    pub nega_scout_search_depth: i32,
}

impl PlayerSettings {
    pub fn create_player(&self) -> Player {
        (self.player_creator.func)(self)
    }
}

impl Default for PlayerSettings {
    #[inline(always)]
    fn default() -> Self {
        PlayerSettings {
            nega_scout_search_depth: 10,
            player_creator: Default::default(),
        }
    }
}
