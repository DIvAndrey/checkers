use egui_macroquad::macroquad::prelude::*;
use crate::constants::*;

pub struct UiParams {
    pub new_scale_coefficient: f32,
    pub curr_scale_coefficient: f32,
    pub target_fps: f64,
    pub white_texture: Texture2D,
    pub white_queen_texture: Texture2D,
    pub black_texture: Texture2D,
    pub black_queen_texture: Texture2D,
    pub board_white_color: Color,
    pub board_black_color: Color,
    pub highlight_color: Color,
    pub hint_color: Color,
    pub eval_bar_white: Color,
    pub eval_bar_black: Color,
    pub eval_bar_gray: Color,
    pub font: Font,
}

impl Default for UiParams {
    fn default() -> Self {
        UiParams {
            new_scale_coefficient: 1.0,
            curr_scale_coefficient: 1.0,
            target_fps: {
                #[cfg(target_arch = "wasm32")]
                {
                    30.0
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    60.0
                }
            },
            white_texture: Texture2D::from_file_with_format(WHITE_CHECKER_IMG, Some(ImageFormat::Png)),
            white_queen_texture: Texture2D::from_file_with_format(
                WHITE_QUEEN_IMG,
                Some(ImageFormat::Png),
            ),
            black_texture: Texture2D::from_file_with_format(BLACK_CHECKER_IMG, Some(ImageFormat::Png)),
            black_queen_texture: Texture2D::from_file_with_format(
                BLACK_QUEEN_IMG,
                Some(ImageFormat::Png),
            ),
            board_white_color: color_u8!(235, 236, 208, 255),
            board_black_color: color_u8!(119, 149, 86, 255),
            highlight_color: color_u8!(42, 71, 173, 100),
            eval_bar_white: color_u8!(235, 235, 240, 255),
            eval_bar_black: color_u8!(50, 48, 49, 255),
            eval_bar_gray: color_u8!(150, 150, 160, 255),
            hint_color: color_u8!(255, 201, 14, 100),
            font: load_ttf_font_from_bytes(FONT).expect("Unable to load font"),
        }
    }
}
