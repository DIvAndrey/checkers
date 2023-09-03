use egui_macroquad::macroquad::time::get_time;

pub struct GameTimer {
    pub delay_between_moves: f64,
    pub last_move_time: f64,
    pub start_time: f64,
}

impl Default for GameTimer {
    fn default() -> Self {
        GameTimer {
            delay_between_moves: 0.0,
            last_move_time: 0.0,
            start_time: 0.0,
        }
    }
}

impl GameTimer {
    #[inline(always)]
    pub fn time_until_last_move(&self) -> f64 {
        self.last_move_time - self.start_time
    }

    #[inline(always)]
    pub fn time_since_last_move(&self) -> f64 {
        get_time() - self.last_move_time
    }
}
