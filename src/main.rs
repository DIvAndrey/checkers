#![windows_subsystem = "windows"]

mod app;

pub mod bot;
pub mod constants;
pub mod game;
pub mod useful_functions;

use egui_macroquad::macroquad;
use egui_macroquad::macroquad::prelude::*;
use app::all_params::scene::Scene;
use crate::app::all_params::AllParams;
use app::{draw_game_frame, draw_menu_frame};
use crate::app::all_params::game_params::player::Player;
use crate::bot::Bot;

fn window_conf() -> Conf {
    Conf {
        window_title: "Checkers".to_string(),
        window_width: 1350,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

async fn draw_frame(params: &mut AllParams, sleep_time: f64) {
    match params.current_scene {
        Scene::Menu => {
            draw_menu_frame(params).await;
            return;
        },
        Scene::Game => draw_game_frame(params).await,
    }
    let start_time = get_time();
    while get_time() - start_time < sleep_time {
        params.evaluation_bar.bot.poll();
        match params.game_params.get_curr_player_mut() {
            Player::Human => {}
            Player::Computer(bot) => {
                bot.poll();
            },
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut params = AllParams::default();
    let mut sleep_time = 0.0f64;
    for _ in 0.. {
        let frame_start_time = get_time();
        draw_frame(&mut params, sleep_time.max(0.004)).await;
        let update_start_time = get_time();
        egui_macroquad::draw();
        next_frame().await;
        let update_duration = get_time() - update_start_time;
        let frame_time = get_time() - frame_start_time;
        if update_duration > frame_time * 0.2 {
            sleep_time += 0.001;
        } else {
            sleep_time -= 0.001;
        }
    }
}

// 14-14 game
// v2: 0.2336
// v3: 0.1726
// v4: 0.1537
// v5: 0.1774, pos eval
// v6: 0.1301, pos eval
// v7: 0.1226, pos eval
// v7: 0.1176, pos eval
