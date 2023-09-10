use lazy_static::lazy_static;
use crate::constants::{MOVES_WITHOUT_CAPTURES_BYTES, MOVES_WITH_CAPTURES_BYTES};

lazy_static! {
    pub static ref MOVES_WITHOUT_CAPTURES: &'static [u64] = {
        assert_eq!(MOVES_WITHOUT_CAPTURES_BYTES.len() % 8, 0);
        unsafe {
            core::slice::from_raw_parts(MOVES_WITHOUT_CAPTURES_BYTES.as_ptr() as *const u64, MOVES_WITHOUT_CAPTURES_BYTES.len() / 8)
        }
    };

    pub static ref MOVES_WITH_CAPTURES: &'static [u64] = {
        assert_eq!(MOVES_WITHOUT_CAPTURES_BYTES.len() % 8, 0);
        unsafe {
            core::slice::from_raw_parts(MOVES_WITH_CAPTURES_BYTES.as_ptr() as *const u64, MOVES_WITH_CAPTURES_BYTES.len() / 8)
        }
    };
}

pub const MAX_POSITION_MAGIC_INDEX: usize = 65536;
pub const MAGIC_RSHIFT: i32 = 48;

pub const MAGIC_NUMBERS: [u64; 64] = [
    0,
    4143770916557949827,
    0,
    5632582401447071962,
    0,
    2953914551181118847,
    0,
    6562325409673774120,
    9585450094518732674,
    0,
    16177932326718848195,
    0,
    5438547638821680427,
    0,
    1316988157189021112,
    0,
    0,
    11236089243641839446,
    0,
    8373016117065810887,
    0,
    18298605313844674498,
    0,
    7774925274806327043,
    11552719122198531437,
    0,
    16355948053622440450,
    0,
    13835257906240287104,
    0,
    7904793738053181469,
    0,
    0,
    16221762135114142392,
    0,
    18131146181627609087,
    0,
    2238605914536407042,
    0,
    10182201213894037784,
    17492436739168537486,
    0,
    8626459604100937254,
    0,
    15234342964417006010,
    0,
    15276907987282906584,
    0,
    0,
    13441332562300502496,
    0,
    17714124635424953016,
    0,
    3760132481320223818,
    0,
    8091113704835661926,
    16791105621719238935,
    0,
    126634105886478339,
    0,
    8485869665191866413,
    0,
    8276781792258883840,
    0,
];
