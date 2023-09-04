use egui_macroquad::egui;
use egui_macroquad::egui::{Align2, Separator, Slider, Window};
use egui_macroquad::macroquad::prelude::*;
use crate::app::all_params::AllParams;
use crate::app::all_params::game_params::player::Player;
use crate::app::all_params::menu_params::player_settings::player_creator::PlayerCreator;
use crate::app::all_params::menu_params::player_settings::PlayerSettings;
use crate::app::{create_delay_between_moves_label, create_delay_between_moves_slider, create_target_fps_label, create_target_fps_slider, create_ui_scale_label, create_ui_scale_slider, prepare_params_for_a_new_game};
use crate::bot::NegaScoutBot;
use crate::constants::CONSTANT_UI_SCALE_COEFFICIENT;
use crate::app::all_params::scene::Scene;

fn create_player_settings_ui(ui: &mut egui::Ui, player_settings: &mut PlayerSettings, label: &str) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.radio_value(
            &mut player_settings.player_creator,
            PlayerCreator::new(|_| Player::Human, 0),
            "Human",
        );
        ui.radio_value(
            &mut player_settings.player_creator,
            PlayerCreator::new(
                |settings| {
                    Player::Computer(Box::new(NegaScoutBot::new(
                        settings.nega_scout_search_depth,
                    )))
                },
                1,
            ),
            "Computer",
        );
    });
    match player_settings.player_creator.bot_type_id {
        1 => {
            ui.horizontal(|ui| {
                ui.label("Search depth");
                ui.add(Slider::new(
                    &mut player_settings.nega_scout_search_depth,
                    1..=15,
                ));
            });
        }
        _ => {}
    }
}

pub async fn draw_menu_frame(params: &mut AllParams) {
    prepare_params_for_a_new_game(params);
    let width = screen_width();
    let height = screen_height();
    let min_res = width.min(height);
    clear_background(LIGHTGRAY);
    egui_macroquad::ui(|egui_ctx| {
        egui_ctx.set_pixels_per_point(min_res * CONSTANT_UI_SCALE_COEFFICIENT * params.ui_params.curr_scale_coefficient);
        Window::new("Checkers")
            .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .show(egui_ctx, |ui| {
                create_player_settings_ui(
                    ui,
                    &mut params.menu_params.player_settings[0],
                    "White checkers:",
                );
                create_player_settings_ui(
                    ui,
                    &mut params.menu_params.player_settings[1],
                    "Black checkers:",
                );
                ui.add(Separator::default().spacing(1.0).shrink(5.0));
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        create_delay_between_moves_label(ui);
                        create_ui_scale_label(ui);
                        create_target_fps_label(ui);
                    });
                    ui.vertical(|ui| {
                        create_delay_between_moves_slider(ui, params);
                        create_ui_scale_slider(ui, params);
                        create_target_fps_slider(ui, params);
                    });
                });
                ui.vertical_centered_justified(|ui| {
                    if ui.small_button("Play!").clicked() {
                        params.game_params.players[0] =
                            params.menu_params.player_settings[0].create_player();
                        params.game_params.players[1] =
                            params.menu_params.player_settings[1].create_player();
                        params.current_scene = Scene::Game;
                    }
                });
            });
    });
}
