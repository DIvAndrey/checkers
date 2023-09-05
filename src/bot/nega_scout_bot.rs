use crate::bot::{Bot, BotState, SearchResult};
use crate::constants::{MAX_NEGA_SCOUT_HASH_MAP_SIZE, HALF_OF_INFINITY, INFINITY, MIN_HASH_MAP_SAVE_SEARCH_DEPTH, MIN_NEGA_SCOUT_YIELD_DEPTH};
use crate::game::Game;
use async_recursion::async_recursion;
use egui_macroquad::egui::epaint::ahash::HashMapExt;
use genawaiter::sync::{Co, GenBoxed};
use genawaiter::GeneratorState;
use rustc_hash::FxHashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ValType {
    Exact,
    Beta,
    Alpha,
    None,
}

#[derive(Clone)]
pub struct HMapGameInfo {
    pub eval: i32,
    pub depth: i32,
    pub val_type: ValType,
    pub move_n: i32,
}

impl HMapGameInfo {
    pub fn new(eval: i32, depth: i32, val_type: ValType, move_n: i32) -> HMapGameInfo {
        HMapGameInfo {
            eval,
            depth,
            val_type,
            move_n,
        }
    }
}

pub struct NegaScoutSearcher {
    pub hash_map: FxHashMap<Game, HMapGameInfo>,
}

impl NegaScoutSearcher {
    pub fn new() -> NegaScoutSearcher {
        NegaScoutSearcher {
            hash_map: FxHashMap::with_capacity(MAX_NEGA_SCOUT_HASH_MAP_SIZE),
        }
    }
}

impl NegaScoutSearcher {
    #[async_recursion]
    async fn search(
        &mut self,
        game: &Game,
        game_move: i32,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        co: &Co<()>,
    ) -> i32 {
        // Checking for depth to keep good performance.
        if depth >= MIN_NEGA_SCOUT_YIELD_DEPTH {
            co.yield_(()).await;
        }
        // Clearing the hashmap if it gets too large.
        if self.hash_map.len() >= MAX_NEGA_SCOUT_HASH_MAP_SIZE {
            let mut games_to_delete = Vec::new();
            for (curr_game, info) in self.hash_map.iter() {
                if info.move_n + info.depth < game_move + depth {
                    games_to_delete.push(curr_game.clone());
                }
            }
            for game in &games_to_delete {
                self.hash_map.remove(game);
            }
            co.yield_(()).await;
            if self.hash_map.len() >= MAX_NEGA_SCOUT_HASH_MAP_SIZE / 2 {
                // Deleting old values from hashmap
                for depth in MIN_HASH_MAP_SAVE_SEARCH_DEPTH.. {
                    let mut games_to_delete = Vec::new();
                    for (curr_game, info) in self.hash_map.iter() {
                        if info.depth <= depth {
                            games_to_delete.push(curr_game.clone());
                        }
                    }
                    for game in &games_to_delete {
                        self.hash_map.remove(game);
                    }
                    if self.hash_map.len() < MAX_NEGA_SCOUT_HASH_MAP_SIZE / 2 {
                        break;
                    }
                    co.yield_(()).await;
                }
            }
        }
        let old_alpha = alpha;
        let mut all_moves = game.get_moves_with_takes();
        if depth <= 0 && all_moves.is_empty() {
            return game.evaluate_for_me();
        }
        // Trying to use a value from hashmap
        if let Some(info) = self.hash_map.get(&game) {
            if info.depth >= depth {
                match info.val_type {
                    ValType::Exact => return info.eval,
                    ValType::Beta => {
                        alpha = alpha.max(info.eval);
                        if alpha >= beta {
                            return alpha;
                        }
                    }
                    ValType::Alpha => {
                        beta = beta.min(info.eval);
                        if alpha >= beta {
                            return beta;
                        }
                    }
                    _ => {}
                }
            }
        }
        if all_moves.is_empty() {
            // If there are no moves with takes
            all_moves = game.get_moves_without_takes();
        }
        if all_moves.is_empty() {
            return -HALF_OF_INFINITY - depth * 100_000 + game.evaluate_for_me();
        }
        let mut score = -INFINITY;
        for curr_move in all_moves {
            if alpha >= beta {
                break;
            }
            let mut game_copy = game.clone();
            game_copy.make_move(curr_move);
            game_copy.change_player();
            let mut tmp = -self
                .search(&game_copy, game_move + 1, depth - 1, -alpha - 1, -alpha, co)
                .await;
            if tmp > alpha && tmp < beta {
                tmp = -self.search(&game_copy, game_move + 1, depth - 1, -beta, -tmp, co).await;
            }
            score = score.max(tmp);
            alpha = alpha.max(score);
        }
        if depth < MIN_HASH_MAP_SAVE_SEARCH_DEPTH {
            return score;
        }
        let curr_val = self.hash_map.get(&game);
        let curr_type = match curr_val {
            None => ValType::None,
            Some(val) => val.val_type,
        };
        if curr_val.is_none() || curr_val.unwrap().depth <= depth {
            if old_alpha < score && score < beta {
                self.hash_map
                    .insert(game.clone(), HMapGameInfo::new(score, depth, ValType::Exact, game_move));
            } else if score >= beta {
                if curr_type != ValType::Exact || curr_val.unwrap().depth != depth {
                    self.hash_map
                        .insert(game.clone(), HMapGameInfo::new(score, depth, ValType::Beta, game_move));
                }
            } else if curr_type != ValType::Exact && curr_type != ValType::Beta {
                self.hash_map
                    .insert(game.clone(), HMapGameInfo::new(score, depth, ValType::Alpha, game_move));
            }
        }
        score
    }

    pub async fn choose_best_move(
        &mut self,
        game: &Game,
        game_move: i32,
        depth: i32,
        co: &Co<()>,
    ) -> SearchResult {
        let all_moves = game.get_moves();
        let mut alpha = -INFINITY;
        let beta = INFINITY;
        let mut score = -INFINITY;
        let mut best_move = None;
        for curr_move in all_moves {
            if alpha >= beta {
                break;
            }
            let mut game_copy = game.clone();
            game_copy.make_move(curr_move.clone());
            game_copy.change_player();
            let mut tmp = -self
                .search(&game_copy, game_move + 1, depth - 1, -alpha - 1, -alpha, co)
                .await;
            if tmp > alpha && tmp < beta {
                tmp = -self.search(&game_copy, game_move + 1, depth - 1, -beta, -tmp, co).await;
            }
            if tmp > score || best_move.is_none() {
                score = tmp;
                best_move = Some(curr_move);
            }
            alpha = alpha.max(tmp);
        }
        SearchResult {
            best_move: best_move.expect("No moves from current position"),
            game_evaluation: score,
        }
    }
}

pub struct NegaScoutBot {
    pub bot: Arc<Mutex<NegaScoutSearcher>>,
    pub state: BotState,
    pub search_depth: i32,
}

impl NegaScoutBot {
    pub fn new(search_depth: i32) -> NegaScoutBot {
        NegaScoutBot {
            bot: Arc::new(Mutex::new(NegaScoutSearcher::new())),
            state: Default::default(),
            search_depth,
        }
    }
}

impl Bot for NegaScoutBot {
    // #[inline(always)]
    fn start_search(&mut self, game: Game, game_move: i32) {
        let search_depth = self.search_depth;
        let bot = self.bot.clone();
        self.state = BotState::Pending(GenBoxed::new_boxed(move |co: Co<()>| async move {
            bot.lock()
                .await
                .choose_best_move(&game, game_move, search_depth, &co)
                .await
        }));
    }

    fn poll(&mut self) -> &BotState {
        match &mut self.state {
            BotState::Pending(generator) => {
                match generator.resume() {
                    GeneratorState::Complete(res) => self.state = BotState::Finished(res),
                    GeneratorState::Yielded(_) => {}
                }
            }
            _ => {}
        }
        &self.state
    }

    fn stop(&mut self) {
        self.state = BotState::NotStarted;
    }

    fn recreate(&mut self) {
        *self = NegaScoutBot::new(self.search_depth);
    }
}

impl PartialEq for NegaScoutBot {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for NegaScoutBot {}
