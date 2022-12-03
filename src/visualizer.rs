use crate::bot::{ThreadBot, INFINITY};
use crate::game::{Checker, Game, Move};
use crate::useful_functions::{conv_1d_to_2d, conv_2d_to_1d, sigmoid};
use egui_macroquad::egui;
use egui_macroquad::egui::{Align2, Pos2, Slider, Window};
use egui_macroquad::macroquad::prelude::*;
use std::collections::{HashMap, HashSet};

const UI_SCALE_COEFF: f32 = 1.0 / 300.0;

#[derive(Clone)]
pub struct GameParams {
    pub game: Game,
    pub last_correct_game_state: Game,
    pub selected_checker: Option<i8>,
    pub full_current_move: Vec<(i8, i8)>,
    pub full_current_move_hash_set: HashSet<i8>,
    pub white_ai_eval: i32,
    pub available_cells_to_move: HashMap<i8, i8>,
    pub end_of_game: bool,
    pub move_n: i32,
}

impl Default for GameParams {
    fn default() -> Self {
        Self {
            game: Game::new(),
            last_correct_game_state: Game::new(),
            selected_checker: None,
            full_current_move: vec![],
            full_current_move_hash_set: Default::default(),
            white_ai_eval: 0,
            available_cells_to_move: Default::default(),
            end_of_game: false,
            move_n: 0,
        }
    }
}

impl GameParams {
    fn update_current_move_hash_set(&mut self) {
        self.full_current_move_hash_set =
            HashSet::from_iter(self.full_current_move.iter().map(|x| x.0));
    }
}

pub struct AllParams {
    pub game_params: GameParams,
    pub history: Vec<GameParams>,
    pub players: [Player; 2],
    pub static_analysis: Player,
    pub static_analysis_depth_step: i32,
    pub static_analysis_start_depth: i32,
    pub static_analysis_depth: i32,
    pub static_evaluation: i32,
    pub last_evaluated_move: i32,
    pub search_depth: i32,
    pub need_hint: bool,
    pub hint: HashSet<i8>,
    pub delay_between_moves: f32,
    pub elapsed_time: f32,
    pub white_texture: Texture2D,
    pub white_queen_texture: Texture2D,
    pub black_texture: Texture2D,
    pub black_queen_texture: Texture2D,
    pub board_white_color: Color,
    pub board_black_color: Color,
    pub highlight_color: Color,
    pub hint_color: Color,
    pub eval_bar_white: Color,
    pub eval_bar_black: Color,
    pub eval_bar_gray: Color,
    pub font: Font,
}

impl AllParams {
    #[inline(always)]
    pub fn complete_full_move(&mut self) {
        self.game_params.selected_checker = None;
        self.game_params.game.change_player();
        self.static_analysis_depth = self.static_analysis_start_depth;
        self.game_params.last_correct_game_state = self.game_params.game.clone();
        self.game_params.move_n += 1;
        self.game_params.end_of_game = self.game_params.game.get_moves().0.is_empty();
        self.need_hint = false;
        self.hint.clear();
        self.elapsed_time = 0.0;
    }
}

#[derive(PartialEq, Eq)]
pub enum Player {
    Human,
    Computer(ThreadBot),
}

impl Player {
    pub fn recreate_bot(&mut self) {
        match self {
            Player::Human => {}
            Player::Computer(bot) => {
                let old_depth = bot.bot.lock().unwrap().search_depth;
                *bot = ThreadBot::new(old_depth);
            }
        }
    }
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
    let board_letters_offset = min_res * 0.006;
    // Static analysis
    if let Player::Computer(bot) = &mut params.static_analysis {
        if bot.is_searching && bot.is_search_ended() {
            if let Some((hint, _, mut score)) = bot.get_search_result() {
                params.hint = HashSet::from_iter(hint.into_iter().map(|x| x.0));
                if !params.game_params.game.current_player {
                    score = -score;
                }
                params.static_evaluation = score;
                params.static_analysis_depth += params.static_analysis_depth_step;
            } else if params.game_params.game.current_player {
                params.static_evaluation = -INFINITY;
            } else {
                params.static_evaluation = INFINITY;
            }
        }
        if !bot.is_searching || params.last_evaluated_move != params.game_params.move_n {
            params.last_evaluated_move = params.game_params.move_n;
            bot.start_search(
                params.game_params.last_correct_game_state.clone(),
                params.static_analysis_depth,
            );
        }
    }
    // Game
    if let (true, Player::Computer(bot)) = (
        !params.game_params.end_of_game,
        &mut params.players[!params.game_params.game.current_player as usize],
    ) {
        // Bot move
        if bot.is_searching && bot.is_search_ended() {
            if params.elapsed_time >= params.delay_between_moves {
                bot.is_searching = false;
                if let Some((best_move, is_cutting, _)) = bot.get_search_result() {
                    params.game_params.full_current_move = best_move.clone();
                    params.game_params.game.make_move((best_move, is_cutting));
                    params.complete_full_move();
                    params.game_params.update_current_move_hash_set();
                    println!("{}. {}", params.game_params.move_n, get_time());
                }
            }
        } else if !bot.is_searching {
            bot.start_search(params.game_params.game.clone(), params.search_depth);
        }
        params.elapsed_time += get_frame_time();
    } else if !params.game_params.end_of_game && is_mouse_button_pressed(MouseButton::Left) {
        // Player move
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
                        params.complete_full_move();
                    } else {
                        if let Some((to, _)) = params.game_params.full_current_move.last() {
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
                            params.complete_full_move();
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
                params.game_params.update_current_move_hash_set();
            }
        }
    }

    // Drawing egui
    egui_macroquad::ui(|egui_ctx| {
        egui_ctx.set_pixels_per_point(min_res * UI_SCALE_COEFF);
        Window::new("Menu")
            .default_pos(Pos2::new(5.0, 30.0))
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.label("Delay between moves (sec)");
                let size = ui
                    .add(Slider::new(&mut params.delay_between_moves, 0.0..=2.0))
                    .rect
                    .size();
                if ui.add_sized(size, egui::Button::new("Restart ↩")).clicked() {
                    prepare_params_for_new_game(params);
                    return;
                }
                if ui
                    .add_sized(size, egui::Button::new("New game ↺"))
                    .clicked()
                {
                    *scene = Scene::NewGameCreation;
                    return;
                }
                if ui.add_sized(size, egui::Button::new("Hint 💡")).clicked() {
                    params.need_hint = !params.need_hint;
                }
                if !params.history.is_empty() {
                    ui.add_sized((size.x, size.y / 4.0), egui::Separator::default());
                    if ui.add_sized(size, egui::Button::new("Back ⬅")).clicked() {
                        params.complete_full_move();
                        params.game_params = params.history.pop().unwrap();
                    }
                }
            });
        if params.game_params.end_of_game {
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

    // Drawing macroquad
    // Static analysis
    let coeff = sigmoid(params.static_evaluation as f32 / 300.0);
    let white_width = board_width * coeff;
    let black_width = board_width * (1.0 - coeff);
    // White bar
    draw_rectangle(x_offset, 0.0, white_width, y_offset, params.eval_bar_white);
    // Black bar
    draw_rectangle(
        x_offset + white_width,
        0.0,
        black_width,
        y_offset,
        params.eval_bar_black,
    );
    // Evaluation text
    let font_size = y_offset * 0.7;
    let eval = (params.static_evaluation as f64 / 100.0).round() as i32;
    let eval_abs = eval.abs();
    let (offset, text) = if eval_abs > 100_000 {
        (1.5, "∞".to_string())
    } else if eval_abs != 0 {
        let res = eval_abs.to_string();
        (res.chars().count() as f32, res)
    } else {
        (0.0, "".to_string())
    };
    let text_x = if eval > 0 {
        x_offset + board_width * 0.005
    } else {
        x_offset + board_width * 0.995 - offset * font_size * 0.95
    };
    draw_text_ex(
        text.to_string().as_str(),
        text_x,
        y_offset - (y_offset - font_size - y_offset * 0.05) * 0.5,
        TextParams {
            font: params.font,
            font_size: font_size as u16,
            color: params.eval_bar_gray,
            ..Default::default()
        },
    );
    // Board
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
        let font_size = min_res * 0.02;
        if x == 0 {
            draw_text_ex(
                (8 - y).to_string().as_str(),
                real_x1 + board_letters_offset,
                real_y1 + font_size + board_letters_offset,
                TextParams {
                    font: params.font,
                    font_size: font_size as u16,
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
                real_x1 + cell_size - font_size - board_letters_offset,
                real_y1 + cell_size - board_letters_offset,
                TextParams {
                    font: params.font,
                    font_size: font_size as u16,
                    color: color2,
                    ..Default::default()
                },
            );
        }
        let as_flat = 63 - i as i8;
        if params
            .game_params
            .full_current_move_hash_set
            .contains(&as_flat)
        {
            draw_rectangle(
                real_x1,
                real_y1,
                cell_size,
                cell_size,
                params.highlight_color,
            );
        }
        if params.need_hint && !params.game_params.end_of_game && params.hint.contains(&as_flat) {
            draw_rectangle(real_x1, real_y1, cell_size, cell_size, params.hint_color);
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
    params.last_evaluated_move = -1;
    params.players[0].recreate_bot();
    params.players[1].recreate_bot();
    params.static_analysis_depth = params.static_analysis_start_depth;
    params.game_params.move_n = 0;
    params.game_params.end_of_game = false;
    params.game_params.game = Game::new();
    params.game_params.last_correct_game_state = Game::new();
    params.game_params.full_current_move.clear();
    params.game_params.available_cells_to_move.clear();
    params.game_params.selected_checker = None;
    params.game_params.full_current_move_hash_set.clear();
    params.history.clear();
    params.hint.clear();
    params.need_hint = false;
    params.elapsed_time = 0.0;
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
                        Player::Computer(ThreadBot::new(params.search_depth)),
                        "Computer",
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Black checkers:");
                    ui.radio_value(&mut params.players[1], Player::Human, "Human");
                    ui.radio_value(
                        &mut params.players[1],
                        Player::Computer(ThreadBot::new(params.search_depth)),
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
