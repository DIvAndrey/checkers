pub const WHITE_CHECKER_IMG: &[u8] = include_bytes!("../data/white.png");
pub const WHITE_QUEEN_IMG: &[u8] = include_bytes!("../data/white_queen.png");
pub const BLACK_CHECKER_IMG: &[u8] = include_bytes!("../data/black.png");
pub const BLACK_QUEEN_IMG: &[u8] = include_bytes!("../data/black_queen.png");
pub const FONT: &[u8] = include_bytes!("../data/font.ttf");
pub const MOVES_WITHOUT_CAPTURES_BYTES: &[u8] = include_bytes!("../data/moves_without_captures.bin");
pub const MOVES_WITH_CAPTURES_BYTES: &[u8] = include_bytes!("../data/moves_with_captures.bin");

#[cfg(target_arch = "wasm32")]
pub const CONSTANT_UI_SCALE_COEFFICIENT: f32 = 1.0 / 500.0;
#[cfg(not(target_arch = "wasm32"))]
pub const CONSTANT_UI_SCALE_COEFFICIENT: f32 = 1.0 / 350.0;

pub const INFINITY: i32 = 1_000_000_000;
pub const HALF_OF_INFINITY: i32 = 500_000_000;

pub const PAWN_COST: i32 = 1000;
pub const QUEEN_COST: i32 = 3000;

pub const MAX_NEGA_SCOUT_HASH_MAP_SIZE: usize = 5_000_000;
pub const MIN_HASH_MAP_SAVE_SEARCH_DEPTH: i32 = 4;
pub const MIN_NEGA_SCOUT_YIELD_DEPTH: i32 = 6;
