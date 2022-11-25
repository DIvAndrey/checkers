use crate::game::{Game, Move};
use std::mem::swap;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use rustc_hash::FxHashMap;

pub const INFINITY: i32 = 1_000_000_000;
pub const HALF_OF_INFINITY: i32 = 500_000_000;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ValType {
    Exact,
    Beta,
    Alpha,
    None,
}

pub struct Bot {
    // game: (score, depth, val_type)
    pub hash_map: FxHashMap<Game, (i32, i32, ValType)>,
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
            hash_map: FxHashMap::default(),
        }
    }
}

impl Bot {
    fn search(&mut self, game: &Game, depth: i32, mut alpha: i32, mut beta: i32, rx: &Receiver<()>) -> Option<i32> {
        if rx.try_recv().is_ok() {
            return None;
        }
        let old_alpha = alpha;
        let mut all_moves = game.get_moves_with_cutting();
        if depth <= 0 && all_moves.is_empty() {
            return Some(if game.current_player {
                game.evaluate()
            } else {
                -game.evaluate()
            });
        }
        if game.current_player {
            if let Some(&(res, d1, val_type)) = self.hash_map.get(&game) {
                if d1 >= depth {
                    match val_type {
                        ValType::Exact => return Some(res),
                        ValType::Beta => {
                            alpha = alpha.max(res);
                            if alpha >= beta {
                                return Some(alpha);
                            }
                        }
                        ValType::Alpha => {
                            beta = beta.min(res);
                            if alpha >= beta {
                                return Some(beta);
                            }
                        }
                        _ => {}
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
            return Some(-HALF_OF_INFINITY - depth * 100_000
                + if game.current_player {
                game.evaluate()
            } else {
                -game.evaluate()
            });
        }
        let mut new_games = all_moves
            .into_iter()
            .map(|m| {
                let mut game_copy = game.clone();
                game_copy.make_move((m.clone(), is_cutting));
                let score = if depth >= 3 {
                    if game.current_player {
                        game_copy.evaluate()
                    } else {
                        -game_copy.evaluate()
                    }
                } else {
                    0
                } + if game_copy.is_queen_checker(m[0].0) {
                    0
                } else {
                    100
                };
                // let score = 0;
                (m, game_copy, -score)
            })
            .collect::<Vec<(Move, Game, i32)>>();
        new_games.sort_unstable_by_key(|x| x.2);
        let mut score = -INFINITY;
        for (_, mut game_copy, _) in new_games {
            if alpha >= beta {
                break;
            }
            game_copy.change_player();
            let mut tmp = -self.search(&game_copy, depth - 1, -alpha - 1, -alpha, rx)?;
            if tmp > alpha && tmp < beta {
                tmp = -self.search(&game_copy, depth - 1, -beta, -tmp, rx)?;
            }
            score = score.max(tmp);
            alpha = alpha.max(score);
        }
        if game.current_player {
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
        Some(score)
    }

    pub fn choose_best_move(&mut self, game: &Game, depth: i32, rx: Receiver<()>) -> Option<(Move, bool, i32)> {
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
            let mut tmp = -self.search(&game_copy, depth - 1, -alpha - 1, -alpha, &rx)?;
            if tmp > alpha && tmp < beta {
                tmp = -self.search(&game_copy, depth - 1, -beta, -tmp, &rx)?;
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

pub struct ThreadBot {
    pub bot: Arc<Mutex<Bot>>,
    pub join: JoinHandle<Option<(Move, bool, i32)>>,
    pub is_searching: bool,
    pub tx: Sender<()>,
}

impl ThreadBot {
    pub fn new() -> ThreadBot {
        ThreadBot {
            bot: Arc::new(Mutex::new(Bot::new())),
            join: thread::spawn(|| None),
            is_searching: false,
            tx: mpsc::channel().0,
        }
    }

    #[inline]
    pub fn start_search(&mut self, game: Game, depth: i32) {
        let _ = self.tx.send(());
        let bot = self.bot.clone();
        let (tx, rx) = mpsc::channel();
        self.tx = tx;
        self.is_searching = true;
        let mut new_thread = thread::spawn(move || bot.lock().unwrap().choose_best_move(&game, depth, rx));
        swap(&mut self.join, &mut new_thread);
    }

    #[inline]
    pub fn is_search_ended(&self) -> bool {
        self.join.is_finished()
    }

    #[inline]
    pub fn get_search_result(&mut self) -> Option<(Move, bool, i32)> {
        self.is_searching = false;
        let mut new_thread = thread::spawn(|| None);
        swap(&mut self.join, &mut new_thread);
        new_thread.join().expect("Thread join error")
    }
}

impl PartialEq for ThreadBot {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for ThreadBot {}


