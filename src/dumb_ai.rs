// use crate::game::{Game, Move};
//
// const GAMES_PER_MOVE: i32 = 5000;
//
// impl Game {
//     pub fn random_game_score(&self) -> bool {
//         let mut game_copy = self.clone();
//         while let Some(_) = game_copy.make_random_move() {
//             game_copy.change_player();
//         }
//         self.current_player != game_copy.current_player
//     }
//
//     pub fn dumb_evaluate_position(&self) -> f64 {
//         let mut sum = 0.0;
//         for _ in 0..GAMES_PER_MOVE {
//             sum += self.random_game_score() as i32 as f64;
//         }
//         sum / GAMES_PER_MOVE as f64
//     }
//
//     pub fn get_dumb_move(&self) -> Option<(Move, bool)> {
//         let mut best_move = None;
//         let mut best_score = -f64::INFINITY;
//         let (all_moves, is_cutting) = self.get_moves();
//         for m in all_moves {
//             let mut game_copy = self.clone();
//             game_copy.make_move((m.clone(), is_cutting));
//             let score = game_copy.dumb_evaluate_position();
//             if score > best_score {
//                 best_score = score;
//                 best_move = Some(m);
//             }
//         }
//         Some((best_move?, is_cutting))
//     }
// }
