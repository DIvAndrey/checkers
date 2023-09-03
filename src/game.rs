use crate::useful_functions::*;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use crate::constants::{PAWN_COST, QUEEN_COST};

#[derive(Clone, Debug)]
pub enum Move {
    Simple(i8, i8),
    Take(Vec<(i8, i8)>)
}

impl Into<Vec<(i8, i8)>> for Move {
    fn into(self) -> Vec<(i8, i8)> {
        match self {
            Move::Simple(a, b) => vec![(a, -1), (b, -1)],
            Move::Take(v) => v,
        }
    }
}

impl Move {
    pub fn clone_vec(&self) -> Vec<(i8, i8)> {
        self.clone().into()
    }
}

#[derive(Clone)]
pub enum Checker {
    Empty,
    White,
    Black,
    WhiteQueen,
    BlackQueen,
}

impl Checker {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Checker::Empty => ".",
            Checker::White => "w",
            Checker::Black => "b",
            Checker::WhiteQueen => "W",
            Checker::BlackQueen => "B",
        })
    }
}

impl Display for Checker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone, Hash)]
pub struct Game {
    // Bitmasks for game field. If a cell is empty, all corresponding bits must be 0.
    pub not_empty: u64,
    pub is_white: u64,
    pub is_queen: u64,
    // `true` if white, `false` if black.
    pub current_player: bool,
    // Game evaluation for white player. Updates dynamically as the game progresses.
    pub eval_white: i32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            not_empty: 0b_0101_0101___1010_1010___0101_0101___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
            is_white:  0b_0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
            is_queen:  0b_0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000,
            current_player: true,
            eval_white: 0,
        }
    }
}

impl Game {
    #[inline(always)]
    pub fn is_empty_cell(&self, i: i8) -> bool {
        return get_bit(self.not_empty, i) == 0;
    }

    #[inline(always)]
    pub fn is_white_checker(&self, i: i8) -> bool {
        return get_bit(self.is_white, i) == 1;
    }

    #[inline(always)]
    pub fn is_queen_checker(&self, i: i8) -> bool {
        return get_bit(self.is_queen, i) == 1;
    }

    #[inline(always)]
    fn clear_cell(&mut self, i: i8) {
        let mask = 1 << i;
        self.not_empty &= !mask;
        self.is_white &= !mask;
        self.is_queen &= !mask;
    }

    #[inline(always)]
    pub fn change_player(&mut self) {
        self.current_player = !self.current_player;
    }

    #[inline(always)]
    pub fn evaluate(&self) -> i32 {
        self.eval_white
    }

    #[inline(always)]
    pub fn evaluate_for_me(&self) -> i32 {
        self.evaluate() * (self.current_player as i32 * 2 - 1)
    }

    pub fn make_pawn_move(&mut self, from: i8, to: i8) {
        let from_mask = 1 << from;
        let to_mask = 1 << to;
        let is_white_from_bit = self.is_white & from_mask;
        let is_queen_from_bit = self.is_queen & from_mask;
        // Copy data to the new cell and clean the old cell
        self.not_empty ^= from_mask ^ to_mask;
        self.is_white ^= (is_white_from_bit >> from << to) ^ is_white_from_bit;
        self.is_queen ^= (is_queen_from_bit >> from << to) ^ is_queen_from_bit;
        self.eval_white += ((to - from) * (is_queen_from_bit == 0) as i8) as i32;
        // Promotion to queen
        if is_queen_from_bit != 0 && (to > 55 && self.current_player || to < 8 && !self.current_player) {
            let player_coeff = ((self.current_player as i32) << 1) - 1;
            self.is_queen ^= to_mask;
            self.eval_white += player_coeff * (QUEEN_COST - PAWN_COST);
            self.eval_white -= to as i32 * player_coeff;
        }
    }

    pub fn make_cutting_move(&mut self, from: i8, to: i8, cut_i: i8) {
        let curr_player_add_eval = if get_bit(self.is_queen, cut_i) == 1 {
            QUEEN_COST
        } else {
            PAWN_COST
        };
        self.eval_white += (((self.current_player as i32) << 1) - 1) * curr_player_add_eval;
        self.make_pawn_move(from, to);
        // Clear the cell with the taken checker
        self.clear_cell(cut_i);
    }

    pub fn make_move(&mut self, move_to_make: Move) {
        match move_to_make {
            Move::Simple(a, b) => self.make_pawn_move(a, b),
            Move::Take(v) => for i in 1..v.len() {
                self.make_cutting_move(v[i - 1].0, v[i].0, v[i].1);
            }
        }
    }

    #[inline(always)]
    fn get_pawns(&self) -> u64 {
        !self.is_queen & self.not_empty
    }

    #[inline(always)]
    fn get_pawns_left_up_takes_mask(&self, current: u64, enemy: u64) -> u64 {
        current & (enemy >> 9) & (!self.not_empty >> 18) & EXCLUDE_2_LEFT_COLUMNS
    }

    #[inline(always)]
    fn get_pawns_right_up_takes_mask(&self, current: u64, enemy: u64) -> u64 {
        current & (enemy >> 7) & (!self.not_empty >> 14) & EXCLUDE_2_RIGHT_COLUMNS
    }

    #[inline(always)]
    fn get_pawns_left_down_takes_mask(&self, current: u64, enemy: u64) -> u64 {
        current & (enemy << 7) & (!self.not_empty << 14) & EXCLUDE_2_LEFT_COLUMNS
    }

    #[inline(always)]
    fn get_pawns_right_down_takes_mask(&self, current: u64, enemy: u64) -> u64 {
        current & (enemy << 9) & (!self.not_empty << 18) & EXCLUDE_2_RIGHT_COLUMNS
    }

    pub fn get_takes_from_cell_rev(&self, i: i8) -> Vec<Vec<(i8, i8)>> {
        let enemy = self.not_empty
            & if self.current_player {
                !self.is_white
            } else {
                self.is_white
            };
        let cell_mask = 1 << i;
        let mut moves = Vec::new();
        if self.is_queen & cell_mask == 0 {
            if self.get_pawns_left_up_takes_mask(cell_mask, enemy) != 0 {
                moves.push((i + 18, i + 9));
            }
            if self.get_pawns_right_up_takes_mask(cell_mask, enemy) != 0 {
                moves.push((i + 14, i + 7));
            }
            if self.get_pawns_left_down_takes_mask(cell_mask, enemy) != 0 {
                moves.push((i - 14, i - 7));
            }
            if self.get_pawns_right_down_takes_mask(cell_mask, enemy) != 0 {
                moves.push((i - 18, i - 9));
            }
        } else {
            let right_up_mask = DIAGONALS_UP_RIGHT[i as usize] & self.not_empty;
            let left_up_mask = DIAGONALS_UP_LEFT[i as usize] & self.not_empty;
            let right_up_cut_mask = right_up_mask & enemy;
            let left_up_cut_mask = left_up_mask & enemy;
            let after = after_bit(cell_mask);
            let before = before_bit(cell_mask);
            // Up right
            let cut_cell_mask = last_bit(before & right_up_cut_mask & EXCLUDE_RIGHT_COLUMN);
            let stop_cell_mask = last_bit(before & right_up_mask & !cut_cell_mask);
            if cut_cell_mask != 0 {
                let i = get_bit_i(cut_cell_mask);
                let j = get_bit_i_or(stop_cell_mask, 64);
                let mut curr = i + 7;
                while curr < j {
                    moves.push((curr, i));
                    if curr & 0b111 == 0 {
                        break;
                    }
                    curr += 7;
                }
            }
            // Down right
            let cut_cell_mask = first_bit_or_0(after & left_up_cut_mask & EXCLUDE_RIGHT_COLUMN);
            let stop_cell_mask = first_bit_or_0(after & left_up_mask & !cut_cell_mask);
            if cut_cell_mask != 0 {
                let i = get_bit_i(cut_cell_mask);
                let j = get_bit_i_or(stop_cell_mask, -1);
                let mut curr = i - 9;
                while curr > j {
                    moves.push((curr, i));
                    if curr & 0b111 == 0 {
                        break;
                    }
                    curr -= 9;
                }
            }
            // Down left
            let cut_cell_mask = first_bit_or_0(after & right_up_cut_mask & EXCLUDE_LEFT_COLUMN);
            let stop_cell_mask = first_bit_or_0(after & right_up_mask & !cut_cell_mask);
            if cut_cell_mask != 7 {
                let i = get_bit_i(cut_cell_mask);
                let j = get_bit_i_or(stop_cell_mask, -1);
                let mut curr = i - 7;
                while curr > j {
                    moves.push((curr, i));
                    if curr & 0b111 == 7 {
                        break;
                    }
                    curr -= 7;
                }
            }
            // Up left
            let cut_cell_mask = last_bit(before & left_up_cut_mask & EXCLUDE_LEFT_COLUMN);
            let stop_cell_mask = last_bit(before & left_up_mask & !cut_cell_mask);
            if cut_cell_mask != 0 {
                let i = get_bit_i(cut_cell_mask);
                let j = get_bit_i_or(stop_cell_mask, 64);
                let mut curr = i + 9;
                while curr < j {
                    moves.push((curr, i));
                    if curr & 0b111 == 7 {
                        break;
                    }
                    curr += 9;
                }
            }
        }
        let mut new_moves: Vec<Vec<(i8, i8)>> = Vec::new();
        for (to, cut_i) in moves {
            let mut game_copy = self.clone();
            game_copy.make_cutting_move(i, to, cut_i);
            let part2 = game_copy.get_takes_from_cell_rev(to);
            if part2.is_empty() {
                new_moves.push(vec![(to, cut_i), (i, -1)]);
            } else {
                new_moves.extend(part2.into_iter().map(|mut m| {
                    m.last_mut().unwrap().1 = cut_i;
                    m.push((i, -1));
                    m
                }));
            }
        }
        new_moves
    }

    #[inline(always)]
    pub fn get_cuts_from_cell(&self, i: i8) -> Vec<Move> {
        let mut res = self.get_takes_from_cell_rev(i);
        res.iter_mut().for_each(|m| m.reverse());
        res.into_iter().map(|m| Move::Take(m)).collect()
    }

    pub fn get_moves_with_takes(&self) -> Vec<Move> {
        let is_mine = if self.current_player {
            self.is_white
        } else {
            !self.is_white
        };
        let my = self.get_pawns() & is_mine;
        let enemy = self.not_empty & !is_mine;
        let empty_cells = !self.not_empty;
        let masks = [
            (
                7,
                (enemy >> 7) & (empty_cells >> 14) & EXCLUDE_2_RIGHT_COLUMNS & my,
            ),
            (
                -9,
                (enemy << 9) & (empty_cells << 18) & EXCLUDE_2_RIGHT_COLUMNS & my,
            ),
            (
                9,
                (enemy >> 9) & (empty_cells >> 18) & EXCLUDE_2_LEFT_COLUMNS & my,
            ),
            (
                -7,
                (enemy << 7) & (empty_cells << 14) & EXCLUDE_2_LEFT_COLUMNS & my,
            ),
        ];
        let mut moves = Vec::new();
        for (add, mut mask) in masks {
            while mask != 0 {
                // i is current cell number
                let last_bit = last_bit(mask);
                let i = get_bit_i(last_bit);
                mask &= !last_bit;
                let from = (i, -1);
                let to = (i + 2 * add, i + add);
                let mut game_copy = self.clone();
                game_copy.make_cutting_move(from.0, to.0, to.1);
                let part2 = game_copy.get_takes_from_cell_rev(i + 2 * add);
                if part2.is_empty() {
                    moves.push(Move::Take(vec![from, to]));
                } else {
                    part2.into_iter().for_each(|mut m| {
                        m.pop();
                        m.push(to);
                        m.push(from);
                        m.reverse();
                        moves.push(Move::Take(m));
                    });
                }
            }
        }
        let mut cells_to_consider = is_mine & self.not_empty & self.is_queen;
        while cells_to_consider != 0 {
            // i is current cell number
            let last_bit = last_bit(cells_to_consider);
            let i = get_bit_i(last_bit);
            cells_to_consider &= !last_bit;
            moves.extend(self.get_cuts_from_cell(i));
        }
        moves
    }

    pub fn get_moves_without_takes(&self) -> Vec<Move> {
        let is_mine = if self.current_player {
            self.is_white
        } else {
            !self.is_white
        };
        let my_pawns = self.get_pawns() & is_mine;
        let empty = !self.not_empty;
        // Pawns
        let masks = if self.current_player {
            [
                (7, (empty >> 7) & EXCLUDE_RIGHT_COLUMN & my_pawns),
                (9, (empty >> 9) & EXCLUDE_LEFT_COLUMN & my_pawns),
            ]
        } else {
            [
                (-7, (empty << 7) & EXCLUDE_LEFT_COLUMN & my_pawns),
                (-9, (empty << 9) & EXCLUDE_RIGHT_COLUMN & my_pawns),
            ]
        };
        let mut moves = Vec::new();
        for (add, mut mask) in masks {
            while mask != 0 {
                // i is current cell number
                let last_bit = last_bit(mask);
                let i = get_bit_i(last_bit);
                mask &= !last_bit;
                moves.push(Move::Simple(i, i + add));
            }
        }
        // Queens
        let mut cells_to_consider = is_mine & self.is_queen;
        while cells_to_consider != 0 {
            // i is current cell number
            let current_mask = last_bit(cells_to_consider);
            let i = get_bit_i(current_mask);
            cells_to_consider &= !current_mask;
            let up_right = DIAGONALS_UP_RIGHT[i as usize];
            let up_left = DIAGONALS_UP_LEFT[i as usize];
            let before = before_bit(current_mask);
            let after = after_bit(current_mask);
            let mut can_move_to = 0;
            let segment1 = up_right & before;
            let segment2 = segment1 & self.not_empty;
            if segment2 != 0 {
                can_move_to |= after_bit(last_bit(segment2)) & segment1;
            } else {
                can_move_to |= segment1;
            }
            let segment1 = up_right & after;
            let segment2 = segment1 & self.not_empty;
            if segment2 != 0 {
                can_move_to |= before_bit(first_bit(segment2)) & segment1;
            } else {
                can_move_to |= segment1;
            }
            let segment1 = up_left & before;
            let segment2 = segment1 & self.not_empty;
            if segment2 != 0 {
                can_move_to |= after_bit(last_bit(segment2)) & segment1;
            } else {
                can_move_to |= segment1;
            }
            let segment1 = up_left & after;
            let segment2 = segment1 & self.not_empty;
            if segment2 != 0 {
                can_move_to |= before_bit(first_bit(segment2)) & segment1;
            } else {
                can_move_to |= segment1;
            }
            while can_move_to != 0 {
                // i is current cell number
                let mask = last_bit(can_move_to);
                can_move_to &= !mask;
                let j = get_bit_i(mask);
                moves.push(Move::Simple(i, j));
            }
        }
        moves
    }

    // bool value is true if player can cut
    pub fn get_moves(&self) -> Vec<Move> {
        let moves_with_cutting = self.get_moves_with_takes();
        if !moves_with_cutting.is_empty() {
            return moves_with_cutting;
        }
        self.get_moves_without_takes()
    }

    pub fn get_data(&self) -> Vec<Vec<Checker>> {
        let mut result = vec![];
        result.resize(8, Vec::with_capacity(8));
        for k in 0..64 {
            let k = 63 - k;
            result[7 - k as usize / 8].push(if self.is_empty_cell(k) {
                Checker::Empty
            } else if self.is_white_checker(k) {
                if self.is_queen_checker(k) {
                    Checker::WhiteQueen
                } else {
                    Checker::White
                }
            } else {
                if self.is_queen_checker(k) {
                    Checker::BlackQueen
                } else {
                    Checker::Black
                }
            });
        }
        result
    }
}

impl PartialEq for Game {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        let res = (self.not_empty, self.is_white, self.is_queen, self.current_player) == (other.not_empty, other.is_white, other.is_queen, other.current_player);
        if res {
            assert_eq!(self.eval_white, other.eval_white);
        }
        res
    }
}

impl Eq for Game {}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}\n{}\n{}'s move",
            "  a b c d e f g h",
            self.get_data()
                .iter()
                .map(|row| row
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" "))
                .enumerate()
                .map(|(i, x)| format!("{} {}", 8 - i, x))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.current_player { "White" } else { "Black" }
        )
    }
}
