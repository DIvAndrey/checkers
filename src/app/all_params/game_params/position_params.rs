use rustc_hash::FxHashSet;
use crate::game::{Game, Winner};

#[derive(Clone)]
pub struct PositionParams {
    pub game: Game,
    pub last_correct_game_state: Game,
    pub selected_checker: Option<i8>,
    pub full_current_move: Vec<i8>,
    pub full_current_move_hash_set: FxHashSet<i8>,
    pub next_possible_cells: FxHashSet<i8>,
    pub selected_move_with_capture: bool,
    pub winner: Option<Winner>,
    pub moves_cnt: i32,
}

impl Default for PositionParams {
    fn default() -> Self {
        Self {
            game: Game::default(),
            last_correct_game_state: Game::default(),
            selected_checker: None,
            full_current_move: vec![],
            full_current_move_hash_set: Default::default(),
            next_possible_cells: Default::default(),
            selected_move_with_capture: false,
            winner: None,
            moves_cnt: 0,
        }
    }
}

impl PositionParams {
    pub(crate) fn update_current_move_hash_set(&mut self) {
        self.full_current_move_hash_set =
            FxHashSet::from_iter(self.full_current_move.clone().into_iter());
    }

    #[inline(always)]
    pub fn complete_full_move(&mut self) {
        self.selected_checker = None;
        self.game.change_player();
        self.last_correct_game_state = self.game.clone();
        self.moves_cnt += 1;
        self.winner = self.game.get_winner();
    }
}

