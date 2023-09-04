use crate::app::all_params::game_params::player::Player;
use crate::app::all_params::game_params::position_params::PositionParams;
use crate::app::all_params::scene::Scene;
use crate::app::all_params::AllParams;
use crate::app::{create_delay_between_moves_label, create_delay_between_moves_slider, create_target_fps_label, create_target_fps_slider, create_ui_scale_label, create_ui_scale_slider, prepare_params_for_a_new_game};
use crate::bot::{Bot, BotState, SearchResult};
use crate::constants::CONSTANT_UI_SCALE_COEFFICIENT;
use crate::game::{Checker, Move};
use crate::useful_functions::conv_2d_to_1d;
use egui::Button;
use egui_macroquad::egui;
use egui_macroquad::egui::epaint::ahash::HashMapExt;
use egui_macroquad::egui::{lerp, Align2, Pos2, Window};
use egui_macroquad::macroquad::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::default::Default;

fn make_bot_move(params: &mut AllParams, search_result: SearchResult) {
    if params.timer.time_since_last_move() >= params.timer.delay_between_moves {
        let position_params = &mut params.game_params.curr_pos_params;
        position_params.full_current_move = search_result.best_move.clone().into();
        position_params.game.make_move(search_result.best_move.clone());
        params.complete_full_move();
        let position_params = &mut params.game_params.curr_pos_params;
        position_params.update_current_move_hash_set();
    }
}

fn poll_bot_and_try_make_move(params: &mut AllParams) {
    match params.game_params.get_curr_player_mut().get_computer_mut().poll() {
        BotState::Pending(_) => {}
        BotState::Finished(search_result) => {
            let search_result = search_result.clone();
            params.game_params.get_curr_player_mut().get_computer_mut().stop();
            make_bot_move(params, search_result);
        }
        BotState::NotStarted => {
            let game = params.game_params.curr_pos_params.game.clone();
            let moves_cnt = params.game_params.curr_pos_params.moves_cnt;
            params
                .game_params
                .get_curr_player_mut()
                .get_computer_mut()
                .start_search(game, moves_cnt);
        }
    }
}

fn update_evaluation_bar(params: &mut AllParams) {
    let bar = &mut params.evaluation_bar;
    bar.displayed_evaluation = lerp(bar.displayed_evaluation..=bar.new_evaluation, 1.0 - 0.1_f32.powf(get_frame_time()));
    if params.game_params.curr_pos_params.end_of_game {
        // End of game.
        params.evaluation_bar.new_evaluation = if params.game_params.curr_pos_params.game.current_player {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        };
        return;
    }
    if params.evaluation_bar.last_evaluated_move != params.game_params.curr_pos_params.moves_cnt {
        params.evaluation_bar.last_evaluated_move = params.game_params.curr_pos_params.moves_cnt;
        params.evaluation_bar.bot.stop();
    }
    match params.evaluation_bar.bot.poll() {
        BotState::Pending(_) => {}
        BotState::Finished(search_result) => {
            let search_result = search_result.clone();
            let player_coeff = params.game_params.curr_pos_params.game.current_player as i32 * 2 - 1;
            params.hint_params.highlighted_cells = FxHashSet::from_iter(search_result.best_move.clone_vec().iter().map(|x| x.0));
            let bar = &mut params.evaluation_bar;
            bar.new_evaluation = (search_result.game_evaluation * player_coeff) as f32 / 1000.0;
            bar.bot.search_depth += 1;
            bar.bot.stop();
        }
        BotState::NotStarted => {
            params.evaluation_bar.bot.start_search(
                params.game_params.curr_pos_params.last_correct_game_state.clone(),
                params.game_params.curr_pos_params.moves_cnt,
            );
        }
    }
}

// Returns: is it required to call `complete_full_move` function
fn process_click_on_board(pos_params: &mut PositionParams, history: &mut Vec<PositionParams>, clicked_cell: i8) -> bool {
    if pos_params.selected_checker.is_some() && pos_params.available_end_and_take_cells.contains_key(&clicked_cell) {
        // Making move
        history.push(pos_params.clone());
        let from: i8 = pos_params.selected_checker.unwrap();
        let to = clicked_cell;
        let cut_i = pos_params.available_end_and_take_cells[&to];
        pos_params.available_end_and_take_cells.clear();
        if cut_i == -1 {
            pos_params.game.make_pawn_move(from, to);
            pos_params.full_current_move = vec![(from, -1), (to, -1)];
            pos_params.update_current_move_hash_set();
            return true;
        } else {
            if let Some((to, _)) = pos_params.full_current_move.last() {
                if !pos_params.game.is_empty_cell(*to) && pos_params.game.is_white_checker(*to) != pos_params.game.current_player {
                    pos_params.full_current_move.clear();
                }
            }
            if pos_params.full_current_move.is_empty() {
                pos_params.full_current_move.push((from, -1));
            }
            pos_params.full_current_move.push((to, cut_i));
            pos_params.game.make_cutting_move(from, to, cut_i);
            let new_cuts = pos_params.game.get_cuts_from_cell(to);
            if new_cuts.is_empty() {
                pos_params.update_current_move_hash_set();
                return true;
            } else {
                pos_params.selected_checker = Some(to);
                for m in new_cuts {
                    let second_cell = match m {
                        Move::Take(v) => (v[1].0, v[1].1),
                        _ => unreachable!(),
                    };
                    pos_params.available_end_and_take_cells.insert(second_cell.0, second_cell.1);
                }
            }
        }
    } else {
        // Selecting move
        let from = clicked_cell;
        let moves: Vec<Move> = pos_params
            .game
            .get_moves()
            .into_iter()
            .filter(|m| match &m {
                Move::Simple(a, _) => *a == from,
                Move::Take(v) => v[0].0 == from,
            })
            .collect();
        pos_params.available_end_and_take_cells = FxHashMap::with_capacity(moves.len());
        for m in moves {
            let second_cell = match m {
                Move::Simple(_, x) => (x, -1),
                Move::Take(v) => (v[1].0, v[1].1),
            };
            pos_params.available_end_and_take_cells.insert(second_cell.0, second_cell.1);
        }
        pos_params.selected_checker = Some(from);
    }
    pos_params.update_current_move_hash_set();
    false
}

pub async fn draw_game_frame(params: &mut AllParams) {
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
    // Updating evaluation bar
    update_evaluation_bar(params);
    // Game
    if let (true, Player::Computer(_)) = (!params.game_params.curr_pos_params.end_of_game, params.game_params.get_curr_player()) {
        poll_bot_and_try_make_move(params);
    } else if !params.game_params.curr_pos_params.end_of_game && is_mouse_button_pressed(MouseButton::Left) {
        // Player move
        let (mouse_x, mouse_y) = mouse_position();
        if mouse_x >= x_offset && mouse_y >= y_offset {
            let x = ((mouse_x - x_offset) / cell_size) as usize;
            let y = ((mouse_y - y_offset) / cell_size) as usize;
            if x < 8 && y < 8 {
                let clicked_cell: i8 = conv_2d_to_1d(x, y);
                if process_click_on_board(&mut params.game_params.curr_pos_params, &mut params.game_params.history, clicked_cell) {
                    params.complete_full_move();
                }
            }
        }
    }

    // Drawing egui
    egui_macroquad::ui(|egui_ctx| {
        egui_ctx.set_pixels_per_point(min_res * CONSTANT_UI_SCALE_COEFFICIENT * params.ui_params.curr_scale_coefficient);
        Window::new("Menu")
            .default_pos(Pos2::new(5.0, 30.0))
            .resizable(false)
            .show(egui_ctx, |ui| {
                create_ui_scale_label(ui);
                let scale_slider = create_ui_scale_slider(ui, params);
                create_delay_between_moves_label(ui);
                create_delay_between_moves_slider(ui, params);
                create_target_fps_label(ui);
                create_target_fps_slider(ui, params);
                let size = scale_slider.rect.size();
                if ui.add_sized(size, Button::new("New game ↩")).clicked() {
                    params.current_scene = Scene::Menu;
                    return;
                }
                if ui.add_sized(size, Button::new("Restart ↺")).clicked() {
                    prepare_params_for_a_new_game(params);
                    return;
                }
                if !params.game_params.history.is_empty() {
                    if ui.add_sized(size, Button::new("Back ⬅")).clicked() {
                        params.complete_full_move();
                        params.game_params.curr_pos_params = params.game_params.history.pop().unwrap();
                    }
                }
                ui.add_sized((size.x, size.y / 4.0), egui::Separator::default());
                if ui.add_sized(size, Button::new("Hint 💡")).clicked() {
                    params.hint_params.need_hint = !params.hint_params.need_hint;
                }
            });
        if params.game_params.curr_pos_params.end_of_game {
            let winner = if params.game_params.curr_pos_params.game.current_player {
                "Black"
            } else {
                "White"
            };
            let game_time = params.timer.time_until_last_move();
            let text = format!(
                "{winner} wins!\nTime spent: {:.2}s\nAvg move: {:.4}s",
                game_time,
                game_time / params.game_params.curr_pos_params.moves_cnt as f64,
            );
            Window::new("")
                .resizable(false)
                .collapsible(false)
                .title_bar(false)
                .auto_sized()
                .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(egui_ctx, |ui| {
                    ui.label(text.as_str());
                    if ui.button("New game").clicked() {
                        params.current_scene = Scene::Menu;
                        for player in &mut params.game_params.players {
                            if let Player::Computer(bot) = player {
                                bot.stop();
                            }
                        }
                        return;
                    }
                });
        }
    });

    // Evaluation bar
    let coeff = (params.evaluation_bar.displayed_evaluation / 4.0).tanh() * 0.5 + 0.5;
    let white_width = board_width * coeff;
    let black_width = board_width - white_width;
    // White bar1
    draw_rectangle(x_offset, 0.0, white_width, y_offset, params.ui_params.eval_bar_white);
    // Black bar
    draw_rectangle(x_offset + white_width, 0.0, black_width, y_offset, params.ui_params.eval_bar_black);
    // Evaluation text
    let eval = params.evaluation_bar.new_evaluation;
    let font_size = (y_offset * 1.8) as u16;
    let eval_abs = eval.abs();
    let text = if eval_abs > 100_000.0 {
        "+∞".to_string()
    } else {
        format!("{:.2}", eval_abs)
    };
    let measurements = measure_text(text.as_str(), Some(params.ui_params.font), font_size, 0.5);
    let text_x = if eval >= 0.0 {
        x_offset + board_width * 0.005
    } else {
        x_offset + board_width * 0.995 - measurements.width
    };
    draw_text_ex(
        text.to_string().as_str(),
        text_x,
        (y_offset + measurements.offset_y) * 0.5,
        TextParams {
            font: params.ui_params.font,
            font_size: font_size as u16,
            font_scale: 0.5,
            color: params.ui_params.eval_bar_gray,
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
            (params.ui_params.board_white_color, params.ui_params.board_black_color)
        } else {
            (params.ui_params.board_black_color, params.ui_params.board_white_color)
        };
        draw_rectangle(real_x1, real_y1, cell_size, cell_size, color1);
        let font_size = min_res * 0.02;
        if x == 0 {
            draw_text_ex(
                (8 - y).to_string().as_str(),
                real_x1 + board_letters_offset,
                real_y1 + font_size + board_letters_offset,
                TextParams {
                    font: params.ui_params.font,
                    font_size: font_size as u16,
                    color: color2,
                    ..Default::default()
                },
            );
        }
        if y == 7 {
            draw_text_ex(
                ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][x].to_string().as_str(),
                real_x1 + cell_size - font_size - board_letters_offset,
                real_y1 + cell_size - board_letters_offset,
                TextParams {
                    font: params.ui_params.font,
                    font_size: font_size as u16,
                    color: color2,
                    ..Default::default()
                },
            );
        }
        let as_flat = 63 - i as i8;
        if params.game_params.curr_pos_params.full_current_move_hash_set.contains(&as_flat) {
            draw_rectangle(real_x1, real_y1, cell_size, cell_size, params.ui_params.highlight_color);
        }
        if params.hint_params.need_hint && !params.game_params.curr_pos_params.end_of_game && params.hint_params.highlighted_cells.contains(&as_flat)
        {
            draw_rectangle(real_x1, real_y1, cell_size, cell_size, params.ui_params.hint_color);
        }
        if params
            .game_params
            .curr_pos_params
            .available_end_and_take_cells
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
    let game_data = params.game_params.curr_pos_params.game.get_data();
    for i in 0..8 {
        for j in 0..8 {
            let texture = match game_data[i][j].clone() {
                Checker::Empty => continue,
                Checker::White => params.ui_params.white_texture,
                Checker::Black => params.ui_params.black_texture,
                Checker::WhiteQueen => params.ui_params.white_queen_texture,
                Checker::BlackQueen => params.ui_params.black_queen_texture,
            };
            let real_x1 = j as f32 * cell_size + x_offset;
            let real_y1 = i as f32 * cell_size + y_offset;
            draw_texture_ex(
                texture,
                real_x1 + texture_draw_offset,
                real_y1 + texture_draw_offset,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(cell_size - 2.0 * texture_draw_offset, cell_size - 2.0 * texture_draw_offset)),
                    ..Default::default()
                },
            );
        }
    }

    draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 20.0, 18.0, BLACK);
}
