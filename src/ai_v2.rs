use crate::game::{Game, Move};
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

pub const INFINITY: i32 = 1_000_000_000;
pub const HALF_OF_INFINITY: i32 = 500_000_000;

#[derive(PartialEq, Eq, Clone, Copy)]
enum ValType {
    Exact,
    Beta,
    Alpha,
    None,
}

pub struct Bot {
    // game: (score, depth, val_type)
    pub hash_map: HashMap<Game, (i32, i32, ValType)>,
    pub join: JoinHandle<Option<(Move, bool, i32)>>,
    pub is_searching: bool,
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
            hash_map: HashMap::new(),
            join: thread::spawn(|| None),
            is_searching: true,
        }
    }
}

impl Bot {
    fn search(&mut self, game: &Game, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
        let old_alpha = alpha;
        let mut all_moves = game.get_moves_with_cutting();
        if depth <= 0 && all_moves.is_empty() {
            return if game.current_player {
                game.evaluate()
            } else {
                -game.evaluate()
            };
        }
        if game.current_player {
            unsafe {
                if let Some(&(res, d1, val_type)) = self.hash_map.get(&game) {
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
        let mut is_cutting = true;
        if all_moves.is_empty() {
            // If can't cut
            all_moves = game.get_moves_without_cutting();
            is_cutting = false;
        }
        if all_moves.is_empty() {
            return -HALF_OF_INFINITY - depth * 100_000
                + if game.current_player {
                    game.evaluate()
                } else {
                    -game.evaluate()
                };
        }
        let mut new_games = all_moves
            .into_iter()
            .map(|m| {
                let mut game_copy = game.clone();
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
            let mut tmp = -self.search(&game_copy, depth - 1, -alpha - 1, -alpha);
            if tmp > alpha && tmp < beta {
                tmp = -self.search(&game_copy, depth - 1, -beta, -tmp);
            }
            score = score.max(tmp);
            alpha = alpha.max(score);
        }
        if game.current_player {
            unsafe {
                let curr_val = self.hash_map.get(&game);
                let curr_type = match curr_val {
                    None => ValType::None,
                    Some(val) => val.2,
                };
                if curr_val.is_none() || curr_val.unwrap().1 <= depth {
                    if old_alpha < score && score < beta {
                        self.hash_map
                            .insert(game.clone(), (score, depth, ValType::Exact));
                    } else if score >= beta {
                        if curr_type != ValType::Exact || curr_val.unwrap().1 != depth {
                            self.hash_map
                                .insert(game.clone(), (score, depth, ValType::Beta));
                        }
                    } else if curr_type != ValType::Exact && curr_type != ValType::Beta {
                        self.hash_map
                            .insert(game.clone(), (score, depth, ValType::Alpha));
                    }
                }
            }
        }
        score
    }

    pub fn choose_best_move(&mut self, game: &Game, depth: i32) -> Option<(Move, bool, i32)> {
        let (all_moves, is_cutting) = game.get_moves();
        let mut alpha = -INFINITY;
        let beta = INFINITY;
        let mut score = -INFINITY;
        let mut best_move = None;
        for curr_move in all_moves {
            if alpha >= beta {
                break;
            }
            let mut game_copy = game.clone();
            game_copy.make_move((curr_move.clone(), is_cutting));
            game_copy.change_player();
            let mut tmp = -self.search(&game_copy, depth - 1, -alpha - 1, -alpha);
            if tmp > alpha && tmp < beta {
                tmp = -self.search(&game_copy, depth - 1, -beta, -tmp);
            }
            if tmp > score || best_move.is_none() {
                score = tmp;
                best_move = Some(curr_move);
            }
            alpha = alpha.max(tmp);
        }
        Some((best_move?, is_cutting, score))
    }

    #[inline]
    pub fn start_search(&mut self, game: Game, depth: i32) {
        self.is_searching = true;
        self.join = thread::spawn(|| self.choose_best_move(&game, depth))
    }

    #[inline]
    pub fn is_search_ended(&self) -> bool {
        self.join.is_finished()
    }
}

impl PartialEq for Bot {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for Bot {}
