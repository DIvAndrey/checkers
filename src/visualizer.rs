use crate::ai_v2::Bot;
use crate::game::{Checker, Game, Move};
use crate::useful_functions::{conv_1d_to_2d, conv_2d_to_1d};
use egui_macroquad::egui;
use egui_macroquad::egui::{Align2, FontFamily, Pos2, Separator, Slider, Style, TextStyle, Window};
use egui_macroquad::macroquad::prelude::*;
use std::collections::{BTreeSet, HashMap};
use std::mem::swap;

const UI_SCALE_COEFF: f32 = 1.0 / 300.0;

fn get_all_moves_string(game: &Game) -> String {
    let moves: Vec<Move> = game.get_moves().0.into_iter().collect();
    let mut filtered = BTreeSet::new();
    for m in moves {
        filtered.insert((conv_1d_to_2d(m[0].0), conv_1d_to_2d(m[1].0)));
    }
    filtered
        .into_iter()
        .map(|(a, b)| {
            format!(
                "{}{} {}{}",
                ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][a.0],
                8 - a.1,
                ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][b.0],
                8 - b.1
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_pretty_score(real_score: i32) -> i32 {
    if real_score > 0 {
        (real_score + 500) / 1000
    } else {
        (real_score - 500) / 1000
    }
}

#[derive(Clone)]
pub struct GameParams {
    pub game: Game,
    pub selected_checker: Option<i8>,
    pub all_moves_string: String,
    pub full_current_move: Vec<(i8, i8)>,
    pub white_ai_eval: i32,
    pub available_cells_to_move: HashMap<i8, i8>,
    pub end_of_game: bool,
    pub move_n: i32,
}

pub struct AllParams {
    pub game_params: GameParams,
    pub history: Vec<GameParams>,
    pub players: [Player; 2],
    pub search_depth: i32,
    pub first_frame: bool,
    pub white_texture: Texture2D,
    pub white_queen_texture: Texture2D,
    pub black_texture: Texture2D,
    pub black_queen_texture: Texture2D,
    pub board_white_color: Color,
    pub board_black_color: Color,
    pub highlight_color: Color,
    pub font: Font,
}

#[derive(PartialEq, Eq)]
pub enum Player {
    Human,
    Computer(Bot),
}

pub enum Scene {
    NewGameCreation,
    Game,
}

pub async fn draw_game_frame(scene: &mut Scene, params: &mut AllParams) {
    clear_background(LIGHTGRAY);
    let width = screen_width();
    let height = screen_height();
    let min_res = width.min(height);
    let y_offset = min_res * 0.03;
    let board_width = min_res - y_offset;
    let x_offset = (width - board_width) / 2.0;
    let cell_size = board_width / 8.0;
    let texture_draw_offset = cell_size * 0.02;
    let hint_circle_radius = cell_size / 2.0 * 0.4;
    if let (true, true, Player::Computer(bot)) = (!params.first_frame, !params.game_params.end_of_game, &mut params.players[!params.game_params.game.current_player as usize])
    {
        if bot.is_searching && bot.is_search_ended() {
            bot.is_searching = false;
            let mut old_bot = Bot::new();
            swap(bot, &mut old_bot);
            if let Some((best_move, is_cutting, _)) = old_bot.join.join().unwrap() {
                params.game_params.full_current_move = best_move.clone();
                params.game_params.game.make_move((best_move, is_cutting));
                params.game_params.game.change_player();
                params.game_params.all_moves_string = get_all_moves_string(&params.game_params.game);
                params.game_params.move_n += 1;
            }
        } else {
            bot.start_search(params.game_params.game.clone(), params.search_depth);
        }
    } else if !params.game_params.end_of_game && is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        if mouse_x >= x_offset && mouse_y >= y_offset {
            let x = ((mouse_x - x_offset) / cell_size) as usize;
            let y = ((mouse_y - y_offset) / cell_size) as usize;
            if x < 8 && y < 8 {
                let to: i8 = conv_2d_to_1d(x, y);
                if params.game_params.selected_checker.is_some()
                    && params.game_params.available_cells_to_move.contains_key(&to)
                {
                    // Making move
                    params.history.push(params.game_params.clone());
                    let from: i8 = params.game_params.selected_checker.unwrap();
                    let cut_i = params.game_params.available_cells_to_move[&to];
                    params.game_params.available_cells_to_move.clear();
                    if cut_i == -1 {
                        params.game_params.game.make_pawn_move(from, to);
                        params.game_params.full_current_move = vec![(from, -1), (to, -1)];
                        params.game_params.selected_checker = None;
                        params.game_params.game.change_player();
                        params.game_params.move_n += 1;
                    } else {
                        if let Some((to, _)) = params.game_params.full_current_move.last() {
                            // if params.game_params.game.get_cuts_from_cell(*to).is_empty()
                            if !params.game_params.game.is_empty_cell(*to)
                                && params.game_params.game.is_white_checker(*to)
                                    != params.game_params.game.current_player
                            {
                                params.game_params.full_current_move.clear();
                            }
                        }
                        if params.game_params.full_current_move.is_empty() {
                            params.game_params.full_current_move.push((from, -1));
                        }
                        params.game_params.full_current_move.push((to, cut_i));
                        params.game_params.game.make_cutting_move(from, to, cut_i);
                        let new_cuts = params.game_params.game.get_cuts_from_cell(to);
                        if new_cuts.is_empty() {
                            params.game_params.selected_checker = None;
                            params.game_params.game.change_player();
                            params.game_params.move_n += 1;
                        } else {
                            params.game_params.selected_checker = Some(to);
                            for m in new_cuts {
                                params
                                    .game_params
                                    .available_cells_to_move
                                    .insert(m[1].0, m[1].1);
                            }
                        }
                    }
                    params.game_params.all_moves_string =
                        get_all_moves_string(&params.game_params.game);
                } else {
                    // Selecting move
                    let moves: Vec<Move> = params
                        .game_params
                        .game
                        .get_moves()
                        .0
                        .into_iter()
                        .filter(|m| conv_1d_to_2d(m[0].0) == (x, y))
                        .collect();
                    params.game_params.available_cells_to_move.clear();
                    for m in moves {
                        params
                            .game_params
                            .available_cells_to_move
                            .insert(m[1].0, m[1].1);
                    }
                    params.game_params.selected_checker = Some(conv_2d_to_1d(x, y));
                }
            }
        }
    }
    params.first_frame = false;
    egui_macroquad::ui(|egui_ctx| {
        egui_ctx.set_pixels_per_point(min_res * UI_SCALE_COEFF);
        Window::new("Menu")
            .default_pos(Pos2::new(5.0, 30.0))
            .resizable(false)
            .show(egui_ctx, |ui| {
                if ui.button("Restart ↩").clicked() {
                    prepare_params_for_new_game(params);
                    return;
                }
                if ui.button("New game ↺").clicked() {
                    *scene = Scene::NewGameCreation;
                    return;
                }
                if !params.history.is_empty() {
                    if ui.button("Back ⬅").clicked() {
                        params.game_params = params.history.pop().unwrap();
                    }
                }
            });
        if params.game_params.all_moves_string.is_empty() {
            params.game_params.end_of_game = true;
            let winner = if params.game_params.game.current_player {
                "Black"
            } else {
                "White"
            };
            let text = format!("{winner} wins!");
            Window::new("")
                .resizable(false)
                .collapsible(false)
                .title_bar(false)
                .auto_sized()
                .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(egui_ctx, |ui| {
                    ui.label(text.as_str());
                    if ui.button("New game").clicked() {
                        *scene = Scene::NewGameCreation;
                        return;
                    }
                });
        }
    });

    for i in 0..64 {
        let x = i % 8;
        let y = i / 8;
        let real_x1 = x as f32 * cell_size + x_offset;
        let real_y1 = y as f32 * cell_size + y_offset;
        let (color1, color2) = if (x + y) % 2 == 0 {
            (params.board_white_color, params.board_black_color)
        } else {
            (params.board_black_color, params.board_white_color)
        };
        draw_rectangle(real_x1, real_y1, cell_size, cell_size, color1);
        if x == 0 {
            draw_text_ex(
                (8 - y).to_string().as_str(),
                real_x1 + 0.005 * min_res,
                real_y1 + 17.0 + 0.005 * min_res,
                TextParams {
                    font: params.font,
                    font_size: 14,
                    color: color2,
                    ..Default::default()
                },
            );
        }
        if y == 7 {
            draw_text_ex(
                ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][x]
                    .to_string()
                    .as_str(),
                real_x1 + cell_size - 15.0 - 0.005 * min_res,
                real_y1 + cell_size - 2.0 - 0.005 * min_res,
                TextParams {
                    font: params.font,
                    font_size: 14,
                    color: color2,
                    ..Default::default()
                },
            );
        }
        if params
            .game_params
            .full_current_move
            .iter()
            .find(|(x, _)| *x == 63 - i as i8)
            .is_some()
        {
            draw_rectangle(
                real_x1,
                real_y1,
                cell_size,
                cell_size,
                params.highlight_color,
            );
        }
        if params
            .game_params
            .available_cells_to_move
            .contains_key(&conv_2d_to_1d(x, y))
        {
            draw_circle(
                real_x1 + cell_size / 2.0,
                real_y1 + cell_size / 2.0,
                hint_circle_radius,
                Color::new(0.1, 0.1, 0.1, 0.17),
            );
        }
    }
    let game_data = params.game_params.game.get_data();
    for i in 0..8 {
        for j in 0..8 {
            let texture = match game_data[i][j].clone() {
                Checker::Empty => continue,
                Checker::White => params.white_texture,
                Checker::Black => params.black_texture,
                Checker::WhiteQueen => params.white_queen_texture,
                Checker::BlackQueen => params.black_queen_texture,
            };
            let real_x1 = j as f32 * cell_size + x_offset;
            let real_y1 = i as f32 * cell_size + y_offset;
            draw_texture_ex(
                texture,
                real_x1 + texture_draw_offset,
                real_y1 + texture_draw_offset,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(
                        cell_size - 2.0 * texture_draw_offset,
                        cell_size - 2.0 * texture_draw_offset,
                    )),
                    ..Default::default()
                },
            );
        }
    }

    draw_text(
        format!("FPS: {}", get_fps()).as_str(),
        10.0,
        20.0,
        18.0,
        BLACK,
    );
    egui_macroquad::draw();
    next_frame().await;
}

pub fn prepare_params_for_new_game(params: &mut AllParams) {
    params.game_params.end_of_game = false;
    params.game_params.game = Game::new();
    params.game_params.full_current_move.clear();
    params.game_params.available_cells_to_move.clear();
    params.game_params.all_moves_string = get_all_moves_string(&params.game_params.game);
    params.first_frame = true;
    params.history.clear();
}

pub async fn new_game(scene: &mut Scene, params: &mut AllParams) {
    prepare_params_for_new_game(params);
    let width = screen_width();
    let height = screen_height();
    let min_res = width.min(height);
    clear_background(LIGHTGRAY);
    egui_macroquad::draw();
    egui_macroquad::ui(|egui_ctx| {
        egui_ctx.set_pixels_per_point(min_res * UI_SCALE_COEFF);
        Window::new("Checkers")
            .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.add(Slider::new(&mut params.search_depth, 1..=15).text("Difficulty level"));
                ui.horizontal(|ui| {
                    ui.label("White checkers:");
                    ui.radio_value(&mut params.players[0], Player::Human, "Human");
                    ui.radio_value(
                        &mut params.players[0],
                        Player::Computer(Bot::new()),
                        "Computer",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Black checkers:");
                    ui.radio_value(&mut params.players[1], Player::Human, "Human");
                    ui.radio_value(
                        &mut params.players[1],
                        Player::Computer(Bot::new()),
                        "Computer",
                    );
                });
                ui.vertical_centered_justified(|ui| {
                    if ui.small_button("Play!").clicked() {
                        *scene = Scene::Game;
                    }
                });
            });
    });
    next_frame().await;
}

pub async fn draw_frame(scene: &mut Scene, params: &mut AllParams) {
    match scene {
        Scene::NewGameCreation => new_game(scene, params).await,
        Scene::Game => draw_game_frame(scene, params).await,
    }
}
