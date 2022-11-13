// use crate::game::{Game, Move};
// use crate::useful_functions::sigmoid;
// use std::collections::HashMap;
//
// const EXPLORATION_COEFFICIENT: f32 = 1.41;
// const EVALUATION_COEFFICIENT: f32 = 0.1;
//
// #[derive(Clone)]
// pub struct Node {
//     pub game: Game,
//     wins: f32,
//     visits: f32,
//     will_win: Option<bool>,
// }
//
// impl Node {
//     pub fn new(game: Game) -> Node {
//         Node {
//             game,
//             wins: 0.0,
//             visits: 0.0,
//             will_win: None,
//         }
//     }
//
//     pub fn get_score(&self, par_visits: f32) -> f32 {
//         if self.visits < 0.1 {
//             // If the node hasn't been visited yet
//             return 1e9;
//         }
//         self.wins / self.visits + EXPLORATION_COEFFICIENT * (par_visits.ln() / self.visits).sqrt()
//     }
// }
//
// pub struct AI {
//     pub nodes: Vec<Node>,
//     reb: Vec<Vec<usize>>,
//     get_node_i: HashMap<Game, usize>,
// }
//
// impl AI {
//     pub fn new(game: Game) -> AI {
//         AI {
//             nodes: vec![Node::new(game)],
//             reb: vec![vec![]],
//             get_node_i: HashMap::new(),
//         }
//     }
//
//     pub fn mcts_step(&mut self) {
//         // Selection
//         let mut path_nodes_indexes = vec![0];
//         let mut node_i = 0;
//         let mut it = 0;
//         while node_i < self.nodes.len() && node_i < self.reb.len() && !self.reb[node_i].is_empty() {
//             let curr_node_visits = self.nodes[node_i].visits;
//             if let Some((new_i, _)) = self.reb[node_i]
//                 .iter()
//                 .map(|i| &self.nodes[*i])
//                 .filter(|node| node.will_win.is_none())
//                 .map(|node| (node, node.get_score(curr_node_visits)))
//                 .enumerate()
//                 .max_by(|(_, (_, s1)), (_, (_, s2))| s1.total_cmp(s2))
//             {
//                 let new_i = self.reb[node_i][new_i];
//                 path_nodes_indexes.push(new_i);
//                 node_i = new_i;
//                 it += 1;
//                 if it > 10000 {
//                     println!("aaaa");
//                 }
//             } else {
//                 self.nodes[node_i].will_win = Some(self.reb[node_i].iter().any(|i| {
//                     if let Some(true) = self.nodes[*i].will_win {
//                         true
//                     } else {
//                         false
//                     }
//                 }));
//                 return;
//             }
//         }
//
//         // Expansion
//         let (all_moves, is_cutting) = self.nodes[node_i].game.get_moves();
//         // Expanding `reb` if it don't have enough len
//         if node_i >= self.reb.len() {
//             self.reb.resize(node_i + 1, Vec::new());
//         }
//         if all_moves.is_empty() {
//             self.nodes[node_i].will_win = Some(false);
//         }
//         // Creating new nodes for each move
//         for curr_move in all_moves {
//             let mut game_copy = self.nodes[node_i].game.clone();
//             game_copy.make_move((curr_move, is_cutting));
//             if let Some(&existing_i) = self.get_node_i.get(&game_copy) {
//                 // This game position already exists in `nodes` vector
//                 self.reb[node_i].push(existing_i);
//             } else {
//                 // Creating new node
//                 let new_node_i = self.nodes.len();
//                 self.reb[node_i].push(new_node_i);
//                 // self.get_node_i.insert(game_copy.clone(), new_node_i);
//                 self.nodes.push(Node::new(game_copy));
//             }
//         }
//
//         // Simulation
//         let mut simulation_res = if let Some(will_win) = self.nodes[node_i].will_win {
//             if (self.nodes[0].game.current_player == self.nodes[node_i].game.current_player) == will_win {
//                 1.0
//             } else {
//                 0.0
//             }
//         } else {
//             // let mut game_copy = self.nodes[node_i].game.clone();
//             // path_nodes_indexes.push(game_copy.make_random_move().unwrap());
//             // loop {
//             //     let end_of_game = game_copy.make_random_move().is_none();
//             //     if end_of_game {
//             //         if self.nodes[0].game.current_player == game_copy.current_player {
//             //             break 0.0;
//             //         } else {
//             //             break 1.0;
//             //         }
//             //     }
//             //     game_copy.change_player();
//             // }
//             if self.nodes[0].game.current_player {
//                 sigmoid(self.nodes[node_i].game.evaluate() as f32 * EVALUATION_COEFFICIENT)
//             } else {
//                 1.0 - sigmoid(self.nodes[node_i].game.evaluate() as f32 * EVALUATION_COEFFICIENT)
//             }
//         };
//
//         // Backpropagation
//         for prev_node_i in path_nodes_indexes {
//             self.nodes[prev_node_i].wins += simulation_res;
//             self.nodes[prev_node_i].visits += 1.0;
//             simulation_res = 1.0 - simulation_res;
//         }
//     }
//
//     pub fn simulate_mcts_steps(&mut self, steps_cnt: i32) {
//         for _ in 0..steps_cnt {
//             self.mcts_step();
//         }
//     }
//
//     pub fn get_best_move_i(&self) -> Option<usize> {
//         Some(
//             self.reb[0]
//                 .iter()
//                 .map(|&i| self.nodes[i].visits)
//                 .enumerate()
//                 .max_by(|(_, a), (_, b)| a.total_cmp(b))?
//                 .0,
//         )
//     }
//
//     pub fn make_move_by_i(&mut self, i: usize, game: &mut Game) -> Move {
//         let (all_moves, is_cutting) = game.get_moves();
//         game.make_move((all_moves[i].clone(), is_cutting));
//         all_moves[i].clone()
//     }
//
//     pub fn make_best_move(&mut self, game: &mut Game) -> Option<Move> {
//         let best_i = self.get_best_move_i()?;
//         Some(self.make_move_by_i(best_i, game))
//     }
// }
