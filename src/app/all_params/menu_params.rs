use player_settings::PlayerSettings;

pub mod player_settings;

pub struct MenuParams {
    pub player_settings: [PlayerSettings; 2],
}

impl Default for MenuParams {
    fn default() -> Self {
        MenuParams {
            player_settings: Default::default(),
        }
    }
}