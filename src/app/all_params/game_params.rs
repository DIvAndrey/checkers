use position_params::PositionParams;
use player::Player;

pub mod position_params;
pub mod player;

#[derive(Default)]
pub struct GameParams {
    pub curr_pos_params: PositionParams,
    pub history: Vec<PositionParams>,
    pub players: [Player; 2],
}

impl GameParams {
    #[inline(always)]
    pub fn get_curr_player(&self) -> &Player {
        &self.players[!self.curr_pos_params.game.current_player as usize]
    }

    #[inline(always)]
    pub fn get_curr_player_mut(&mut self) -> &mut Player {
        &mut self.players[!self.curr_pos_params.game.current_player as usize]
    }
}
