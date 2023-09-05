#![windows_subsystem = "windows"]

mod app;

pub mod bot;
pub mod constants;
pub mod game;
pub mod useful_functions;

use egui_macroquad::macroquad;
use egui_macroquad::macroquad::prelude::*;
use instant::Instant;
use app::all_params::scene::Scene;
use crate::app::all_params::AllParams;
use app::{draw_game_frame, draw_menu_frame};
use crate::app::all_params::game_params::player::Player;
use crate::bot::{Bot, BotState};

fn window_conf() -> Conf {
    Conf {
        window_title: "Checkers".to_string(),
        window_width: 1350,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

async fn draw_frame(params: &mut AllParams) {
    match params.current_scene {
        Scene::Menu => {
            draw_menu_frame(params).await;
            return;
        },
        Scene::Game => draw_game_frame(params).await,
    }
    let sleep_time = 1.0 / params.ui_params.target_fps;
    let timer = Instant::now();
    while timer.elapsed().as_secs_f64() < sleep_time {
        params.evaluation_bar.bot.poll();
        match params.game_params.get_curr_player_mut() {
            Player::Human => return,
            Player::Computer(bot) => {
                match bot.poll() {
                    BotState::Pending(_) => {}
                    _ => return,
                }
            },
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut params = AllParams::default();
    for _ in 0.. {
        draw_frame(&mut params).await;
        egui_macroquad::draw();
        next_frame().await;
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
