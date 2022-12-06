#![windows_subsystem = "windows"]

extern crate core;

mod bot;
mod game;
mod useful_functions;
mod visualizer;

use crate::bot::ThreadBot;
use crate::visualizer::{draw_frame, AllParams, GameParams, Player, Scene};
use egui_macroquad::macroquad;
use egui_macroquad::macroquad::prelude::*;

const WHITE_BYTES: &[u8] = include_bytes!("../data/white.png");
const WHITE_QUEEN_BYTES: &[u8] = include_bytes!("../data/white_queen.png");
const BLACK_BYTES: &[u8] = include_bytes!("../data/black.png");
const BLACK_QUEEN_BYTES: &[u8] = include_bytes!("../data/black_queen.png");
const FONT_BYTES: &[u8] = include_bytes!("../data/font.ttf");

fn window_conf() -> Conf {
    Conf {
        window_title: "Checkers".to_string(),
        window_width: 1350,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    useful_functions::initialize();
    let mut game_params = AllParams {
        game_params: GameParams::default(),
        white_texture: Texture2D::from_file_with_format(WHITE_BYTES, Some(ImageFormat::Png)),
        white_queen_texture: Texture2D::from_file_with_format(
            WHITE_QUEEN_BYTES,
            Some(ImageFormat::Png),
        ),
        black_texture: Texture2D::from_file_with_format(BLACK_BYTES, Some(ImageFormat::Png)),
        black_queen_texture: Texture2D::from_file_with_format(
            BLACK_QUEEN_BYTES,
            Some(ImageFormat::Png),
        ),
        delay_between_moves: 0.3,
        elapsed_time: 0.0,
        board_white_color: color_u8!(235, 236, 208, 255),
        board_black_color: color_u8!(119, 149, 86, 255),
        highlight_color: color_u8!(42, 71, 173, 100),
        eval_bar_white: color_u8!(235, 235, 240, 255),
        eval_bar_black: color_u8!(50, 48, 49, 255),
        eval_bar_gray: color_u8!(150, 150, 160, 255),
        hint_color: color_u8!(255, 201, 14, 100),
        font: load_ttf_font_from_bytes(FONT_BYTES).expect("Не удалось загрузить шрифт"),
        history: Vec::new(),
        players: [Player::Human, Player::Human],
        static_analysis: Player::Computer(ThreadBot::new(1)),
        static_analysis_depth_step: 2,
        static_analysis_depth: 1,
        static_evaluation: 0,
        static_analysis_start_depth: 6,
        search_depth: 12,
        last_evaluated_move: -1,
        need_hint: false,
        hint: Default::default(),
    };
    let mut game_scene = Scene::NewGameCreation;

    for _ in 0.. {
        draw_frame(&mut game_scene, &mut game_params).await;
    }
}
