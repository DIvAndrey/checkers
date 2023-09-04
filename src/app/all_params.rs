pub mod evaluation_bar;
pub mod game_hint;
pub mod game_params;
pub mod game_timer;
pub mod menu_params;
pub mod ui_params;
pub mod scene;

use egui_macroquad::macroquad::prelude::get_time;
use crate::Scene;
use evaluation_bar::EvaluationBar;
use game_hint::GameHint;
use game_params::GameParams;
use game_timer::GameTimer;
use menu_params::MenuParams;
use ui_params::UiParams;

pub struct AllParams {
    pub menu_params: MenuParams,
    pub game_params: GameParams,
    pub evaluation_bar: EvaluationBar,
    pub timer: GameTimer,
    pub hint_params: GameHint,
    pub ui_params: UiParams,
    pub current_scene: Scene,
}

impl AllParams {
    #[inline(always)]
    pub fn complete_full_move(&mut self) {
        self.game_params.curr_pos_params.complete_full_move();
        self.hint_params.need_hint = false;
        self.hint_params.highlighted_cells.clear();
        self.timer.last_move_time = get_time();
        self.evaluation_bar.bot.search_depth = 1;
    }
}

impl Default for AllParams {
    fn default() -> Self {
        AllParams {
            menu_params: Default::default(),
            game_params: Default::default(),
            evaluation_bar: Default::default(),
            timer: Default::default(),
            hint_params: Default::default(),
            ui_params: Default::default(),
            current_scene: Scene::Menu,
        }
    }
}
