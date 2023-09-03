pub mod all_params;
pub mod game_scene;
pub mod menu_scene;

use crate::app::all_params::AllParams;
use egui_macroquad::egui;
use egui_macroquad::egui::{Response, Slider};
use egui_macroquad::macroquad::time::get_time;
pub use game_scene::draw_game_frame;
pub use menu_scene::draw_menu_frame;
use std::default::Default;

pub fn prepare_params_for_a_new_game(params: &mut AllParams) {
    params.evaluation_bar = Default::default();
    params.game_params.players[0].recreate_bot();
    params.game_params.players[1].recreate_bot();
    params.game_params.curr_pos_params = Default::default();
    params.game_params.history.clear();
    params.hint_params.highlighted_cells.clear();
    params.hint_params.need_hint = false;
    params.timer.start_time = get_time();
}

fn create_delay_between_moves_label(ui: &mut egui::Ui) -> Response {
    ui.label("Delay between moves (sec)")
}

fn create_delay_between_moves_slider(ui: &mut egui::Ui, params: &mut AllParams) -> Response {
    ui.add(Slider::new(
        &mut params.timer.delay_between_moves,
        0.0..=1.0,
    ))
}

fn create_ui_scale_label(ui: &mut egui::Ui) -> Response {
    ui.label("Ui scale")
}

fn create_ui_scale_slider(ui: &mut egui::Ui, params: &mut AllParams) -> Response {
    let response = ui.add(Slider::new(&mut params.ui_params.new_scale_coefficient, 0.3..=2.0));
    if response.drag_released() {
        params.ui_params.curr_scale_coefficient = params.ui_params.new_scale_coefficient;
    }
    response
}
