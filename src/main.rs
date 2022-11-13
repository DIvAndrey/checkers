#![windows_subsystem = "windows"]

extern crate core;

mod ai;
mod ai_v2;
mod dumb_ai;
mod game;
mod useful_functions;
mod visualizer;

use crate::game::Game;
use crate::visualizer::{draw_frame, GameParams, Scene};
use egui_macroquad::macroquad;
use egui_macroquad::macroquad::prelude::*;
use std::collections::HashMap;

const WHITE_BYTES: &[u8] = include_bytes!("../data/white.png");
const WHITE_QUEEN_BYTES: &[u8] = include_bytes!("../data/white_queen.png");
const BLACK_BYTES: &[u8] = include_bytes!("../data/black.png");
const BLACK_QUEEN_BYTES: &[u8] = include_bytes!("../data/black_queen.png");
const FONT_BYTES: &[u8] = include_bytes!("../data/font.ttf");

fn window_conf() -> Conf {
    Conf {
        window_title: "Checkers".to_owned(),
        window_width: 1350,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    useful_functions::initialize();
    let game = Game::new();
    let mut game_params = GameParams {
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
        board_white_color: Color::from_rgba(235, 236, 208, 255),
        board_black_color: Color::from_rgba(119, 149, 86, 255),
        highlight_color: Color::from_rgba(42, 71, 173, 100),
        font: load_ttf_font_from_bytes(FONT_BYTES).expect("Не удалось загрузить шрифт"),
        game,
        available_cells_to_move: HashMap::new(),
        selected_checker: None,
        all_moves_string: "".to_string(),
        full_current_move: vec![],
        white_ai_eval: 0,
        end_of_game: false,
        move_n: 0,
        is_ai_player: [false, true],
        search_depth: 12,
        first_frame: true,
    };
    let mut game_scene = Scene::NewGameCreation;

    // 13 - 67. 6.4720618, time per move: 0.09659793731343283
    // 14 - 69. 27.528797, time per move: 0.39896807246376814

    for _ in 0.. {
        draw_frame(&mut game_scene, &mut game_params).await;
    }
}
