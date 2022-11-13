use crate::game::{Game, Move};
use std::collections::HashMap;

pub const INFINITY: i32 = 1_000_000_000;
pub const HALF_OF_INFINITY: i32 = 500_000_000;
// game: (score, alpha, beta, depth)
static mut HASH_MAP: Option<HashMap<Game, (i32, i32, ValType)>> = None;

#[derive(PartialEq, Eq, Clone, Copy)]
enum ValType {
    Exact,
    Beta,
    Alpha,
    None,
}

impl Game {
    fn search_v7(&self, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
        let old_alpha = alpha;
        let mut all_moves = self.get_moves_with_cutting();
        if depth <= 0 && all_moves.is_empty() {
            return if self.current_player {
                self.evaluate()
            } else {
                -self.evaluate()
            };
        }
        if self.current_player {
            unsafe {
                if let Some(hash_map) = &HASH_MAP {
                    if let Some(&(res, d1, val_type)) = hash_map.get(&self) {
                        if d1 >= depth {
                            match val_type {
                                ValType::Exact => return res,
                                ValType::Beta => {
                                    alpha = alpha.max(res);
                                    if alpha >= beta {
                                        return alpha;
                                    }
                                }
                                ValType::Alpha => {
                                    beta = beta.min(res);
                                    if alpha >= beta {
                                        return beta;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        let mut is_cutting = true;
        if all_moves.is_empty() {
            // If can't cut
            all_moves = self.get_moves_without_cutting();
            is_cutting = false;
        }
        if all_moves.is_empty() {
            return -HALF_OF_INFINITY - depth * 100_000
                + if self.current_player {
                self.evaluate()
            } else {
                -self.evaluate()
            };
        }
        let mut new_games = all_moves
            .into_iter()
            .map(|m| {
                let mut game_copy = self.clone();
                game_copy.make_move((m.clone(), is_cutting));
                (m, game_copy)
            })
            .collect::<Vec<(Move, Game)>>();
        new_games.sort_by_key(
            |(m, game)| {
                if game.is_queen_checker(m[0].0) {
                    2
                } else {
                    1
                }
            },
        );
        let mut score = -INFINITY;
        for (_, mut game_copy) in new_games {
            if alpha >= beta {
                break;
            }
            game_copy.change_player();
            let mut tmp = -game_copy.search_v7(depth - 1, -alpha - 1, -alpha);
            if tmp > alpha && tmp < beta {
                tmp = -game_copy.search_v7(depth - 1, -beta, -tmp);
            }
            score = score.max(tmp);
            alpha = alpha.max(score);
        }
        if self.current_player {
            unsafe {
                if let Some(hash_map) = &mut HASH_MAP {
                    let curr_val = hash_map.get(&self);
                    let curr_type = match curr_val {
                        None => ValType::None,
                        Some(val) => val.2,
                    };
                    if curr_val.is_none() || curr_val.unwrap().1 <= depth {
                        if old_alpha < score && score < beta {
                            hash_map.insert(self.clone(), (score, depth, ValType::Exact));
                        } else if score >= beta {
                            if curr_type != ValType::Exact || curr_val.unwrap().1 != depth {
                                hash_map.insert(self.clone(), (score, depth, ValType::Beta));
                            }
                        } else if curr_type != ValType::Exact && curr_type != ValType::Beta {
                            hash_map.insert(self.clone(), (score, depth, ValType::Alpha));
                        }
                    }
                }
            }
        }
        score
    }

    pub fn choose_best_move_v7(&self, depth: i32) -> Option<(Move, bool, i32)> {
        unsafe {
            if HASH_MAP.is_none() {
                HASH_MAP = Some(HashMap::new());
            }
        }
        let (all_moves, is_cutting) = self.get_moves();
        let mut alpha = -INFINITY;
        let beta = INFINITY;
        let mut score = -INFINITY;
        let mut best_move = None;
        for curr_move in all_moves {
            if alpha >= beta {
                break;
            }
            let mut game_copy = self.clone();
            game_copy.make_move((curr_move.clone(), is_cutting));
            game_copy.change_player();
            let mut tmp = -game_copy.search_v7(depth - 1, -alpha - 1, -alpha);
            if tmp > alpha && tmp < beta {
                tmp = -game_copy.search_v7(depth - 1, -beta, -tmp);
            }
            if tmp > score || best_move.is_none() {
                score = tmp;
                best_move = Some(curr_move);
            }
            alpha = alpha.max(tmp);
        }
        Some((best_move?, is_cutting, score))
    }
}
