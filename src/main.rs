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
    let start_time = get_time();
    while get_time() - start_time < sleep_time {
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

// 1-13
// 17.33 0.3767
// 11.84 0.2575
// 12.09 0.2629
// 6.52  0.1303

//
// use std::fs::File;
// use std::io::{Read, Write};
// use egui_macroquad::egui::epaint::ahash::HashSetExt;
// use egui_macroquad::macroquad::rand::gen_range;
// use crate::useful_functions::{conv_1d_to_2d, conv_2d_to_1d, DIAGONALS, get_bit};
//
// mod useful_functions;
// // mod game;
//
// fn process_case(mask: u64, i: i8) -> u64 {
//     // // No captures
//     // let (mut x1, mut y1) = conv_1d_to_2d(i);
//     // let mut result = 0;
//     // for (dx, dy) in vec![(1, 1), (1, -1), (-1, -1), (-1, 1)] {
//     //     let (mut x, mut y) = (x1 as i8, y1 as i8);
//     //     for _ in 0..8 {
//     //         x += dx;
//     //         y += dy;
//     //         if x > 7 || y > 7 || x < 0 || y < 0 {
//     //             break;
//     //         }
//     //         let cell_i = conv_2d_to_1d(x as usize, y as usize);
//     //         if get_bit(mask, cell_i) == 1 {
//     //             break;
//     //         }
//     //         result |= 1 << cell_i;
//     //     }
//     // }
//     // // println!("{:b}", result);
//     // result & !(1 << i)
//     // Captures
//     assert_eq!(get_bit(mask, i), 0);
//     let (mut x1, mut y1) = conv_1d_to_2d(i);
//     let mut result = 0;
//     for (dx, dy) in vec![(1, 1), (1, -1), (-1, -1), (-1, 1)] {
//         let (mut x, mut y) = (x1 as i8, y1 as i8);
//         let mut has_taken = false;
//         for _ in 0..8 {
//             x += dx;
//             y += dy;
//             if x > 7 || y > 7 || x < 0 || y < 0 {
//                 break;
//             }
//             let cell_i = conv_2d_to_1d(x as usize, y as usize);
//             if get_bit(mask, cell_i) == 1 {
//                 if has_taken {
//                     break;
//                 } else {
//                     has_taken = true;
//                 }
//             } else if has_taken {
//                 result |= 1 << cell_i;
//             }
//         }
//     }
//     result & !(1 << i)
// }
//
// const VEC: Vec<u64> = vec![];
// pub const MAGIC_NUMBERS: [u64; 64] = [
//     0,
//     4143770916557949827,
//     0,
//     5632582401447071962,
//     0,
//     2953914551181118847,
//     0,
//     6562325409673774120,
//     9585450094518732674,
//     0,
//     16177932326718848195,
//     0,
//     5438547638821680427,
//     0,
//     1316988157189021112,
//     0,
//     0,
//     11236089243641839446,
//     0,
//     8373016117065810887,
//     0,
//     18298605313844674498,
//     0,
//     7774925274806327043,
//     11552719122198531437,
//     0,
//     16355948053622440450,
//     0,
//     13835257906240287104,
//     0,
//     7904793738053181469,
//     0,
//     0,
//     16221762135114142392,
//     0,
//     18131146181627609087,
//     0,
//     2238605914536407042,
//     0,
//     10182201213894037784,
//     17492436739168537486,
//     0,
//     8626459604100937254,
//     0,
//     15234342964417006010,
//     0,
//     15276907987282906584,
//     0,
//     0,
//     13441332562300502496,
//     0,
//     17714124635424953016,
//     0,
//     3760132481320223818,
//     0,
//     8091113704835661926,
//     16791105621719238935,
//     0,
//     126634105886478339,
//     0,
//     8485869665191866413,
//     0,
//     8276781792258883840,
//     0,
// ];
//
//
// fn main() {
//     let mut result = vec![0u64; 64 * 65536];
//     for coord in 0_i8..64 {
//         let (x, y) = (coord % 8, coord / 8);
//         if (x + y) % 2 == 0 {
//             continue;
//         }
//         let diagonals = DIAGONALS[coord as usize];
//         let mut ones = Vec::new();
//         for i in 0..64 {
//             if (diagonals >> i) & 1 == 1 {
//                 ones.push(i);
//             }
//         }
//         // dbg!(&ones);
//         let mut all_cases = Vec::with_capacity(1 << ones.len());
//         for mask in 0..(1 << ones.len()) {
//             let mut case: u64 = 0;
//             for i in 0..ones.len() {
//                 if (mask >> i) & 1 == 1 {
//                     case |= 1 << ones[i];
//                 }
//             }
//             if coord == 1 && case == 0b10000000010000000000000000000000000010000000010100000000 {
//                 println!("Found! {}", case.wrapping_mul(MAGIC_NUMBERS[coord as usize]) >> 48);
//             }
//             all_cases.push((case, process_case(case, coord)));
//         }
//         let mut best = (u64::MAX, 46, 0);
//         // let mut i = 0;
//         // while i < 100_000 || best.0 > 100_000 && i < 1_000_000 {
//         //     i += 1;
//         // {
//         // let magic_num = fastrand::u64(1..=u64::MAX);
//         let magic_num = MAGIC_NUMBERS[coord as usize];
//         let mut curr_max_val = 0;
//         for &(pos, _) in all_cases.iter() {
//             let res = pos.wrapping_mul(magic_num) >> 48;
//             curr_max_val = curr_max_val.max(res);
//         }
//         best = best.min((curr_max_val, 48, magic_num));
//         // let mut min_shift = 48;
//         // let mut max_shift = 48;
//         // 'b: while min_shift <= max_shift {
//         //     let shift = (min_shift + max_shift + 1) / 2;
//         //     let mut used = [false; 70_000];
//         //     let mut curr_max_val = 0;
//         //     for &(pos, _) in all_cases.iter() {
//         //         let res = pos.wrapping_mul(magic_num) >> shift;
//         //         if res >= 70_000 {
//         //             min_shift = shift + 1;
//         //             continue 'b;
//         //         }
//         //         if used[res as usize] {
//         //             max_shift = shift - 1;
//         //             continue 'b;
//         //         }
//         //         used[res as usize] = true;
//         //         curr_max_val = curr_max_val.max(res);
//         //     }
//         //     best = best.min((curr_max_val, shift, magic_num));
//         //     min_shift = shift;
//         //     if min_shift == max_shift {
//         //         break;
//         //     }
//         // }
//         // }
//         println!("{}", best.0);
//         let coord = coord as usize;
//         for &(pos, ans) in all_cases.iter() {
//             result[coord * 65536 + (pos.wrapping_mul(best.2) >> best.1) as usize] = ans;
//         }
//     }
//     let mut file = File::create(r"C:\Scripts\CLionProjects\rust_checkers-main\data\moves_with_captures.bin").unwrap();
//     let bytes = bytemuck::cast_slice(result.as_slice());
//     file.write(bytes).unwrap();
// }
//
//
// // 23062