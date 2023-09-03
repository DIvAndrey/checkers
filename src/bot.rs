pub mod nega_scout_bot;

pub use nega_scout_bot::NegaScoutBot;
use crate::game::{Game, Move};
use genawaiter::sync::GenBoxed;

#[derive(Clone)]
pub struct SearchResult {
    pub best_move: Move,
    pub game_evaluation: i32,
}

pub enum BotState {
    NotStarted,
    Pending(GenBoxed<(), (), SearchResult>),
    Finished(SearchResult)
}

impl Default for BotState {
    fn default() -> Self {
        BotState::NotStarted
    }
}

pub trait Bot {
    fn start_search(&mut self, game: Game, game_move: i32);

    fn poll(&mut self) -> &BotState;

    fn stop(&mut self);

    fn recreate(&mut self);
}
