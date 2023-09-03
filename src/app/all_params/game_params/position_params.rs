use rustc_hash::{FxHashSet as HashSet, FxHashMap as HashMap};
use crate::game::Game;

#[derive(Clone)]
pub struct PositionParams {
    pub game: Game,
    pub last_correct_game_state: Game,
    pub selected_checker: Option<i8>,
    pub full_current_move: Vec<(i8, i8)>,
    pub full_current_move_hash_set: HashSet<i8>,
    pub white_ai_eval: i32,
    pub available_end_and_take_cells: HashMap<i8, i8>,
    pub end_of_game: bool,
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
            white_ai_eval: 0,
            available_end_and_take_cells: Default::default(),
            end_of_game: false,
            moves_cnt: 0,
        }
    }
}

impl PositionParams {
    pub(crate) fn update_current_move_hash_set(&mut self) {
        self.full_current_move_hash_set =
            HashSet::from_iter(self.full_current_move.iter().map(|x| x.0));
    }

    #[inline(always)]
    pub fn complete_full_move(&mut self) {
        self.selected_checker = None;
        self.game.change_player();
        self.last_correct_game_state = self.game.clone();
        self.moves_cnt += 1;
        self.end_of_game = self.game.get_moves().is_empty();
    }
}

