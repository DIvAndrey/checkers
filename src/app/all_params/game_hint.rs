use rustc_hash::FxHashSet;

pub struct GameHint {
    pub need_hint: bool,
    pub highlighted_cells: FxHashSet<i8>,
}

impl Default for GameHint {
    fn default() -> Self {
        GameHint {
            need_hint: false,
            highlighted_cells: Default::default(),
        }
    }
}
