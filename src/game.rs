mod magic_numbers;
use crate::useful_functions::*;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::constants::{PAWN_COST, QUEEN_COST};
use crate::game::magic_numbers::{MAGIC_NUMBERS, MAGIC_RSHIFT, MAX_POSITION_MAGIC_INDEX, MOVES_WITH_CAPTURES, MOVES_WITHOUT_CAPTURES};

#[derive(Clone, Debug)]
pub enum Move {
    Simple(i8, i8),
    Take(Vec<i8>)
}

#[derive(Clone, Copy)]
pub enum Winner {
    White,
    Black,
    Draw,
}

impl ToString for Winner {
    fn to_string(&self) -> String {
        match self {
            Winner::White => "White won!",
            Winner::Black => "Black won!",
            Winner::Draw => "Draw",
        }.to_string()
    }
}

impl Into<Vec<i8>> for Move {
    fn into(self) -> Vec<i8> {
        match self {
            Move::Simple(a, b) => vec![a, b],
            Move::Take(v) => v,
        }
    }
}

impl Move {
    pub fn as_vec(&self) -> Vec<i8> {
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

#[derive(Clone)]
pub struct Game {
    // Bitmasks for game field. If a cell is empty, all corresponding bits must be 0.
    pub not_empty: u64,
    pub is_white: u64,
    pub is_queen: u64,
    // Game evaluation for white player. Updates dynamically as the game progresses.
    pub eval_white: i32,
    pub boring_moves_counter: u8,
    // `true` if white, `false` if black.
    pub current_player: bool,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            not_empty: 0b_0101_0101___1010_1010___0101_0101___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
            is_white:  0b_0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
            is_queen:  0b_0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000,
            // is_queen:  0b_0101_0101___1010_1010___0101_0101___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
            eval_white: 0,
            boring_moves_counter: 0,
            current_player: true,
        }
    }
}

impl Hash for Game {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.not_empty);
        state.write_u64(self.is_white);
        state.write_u64(self.is_queen);
        state.write_u8(self.current_player as u8);
        state.write_u8(self.boring_moves_counter);

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

    #[inline(always)]
    pub fn is_draw(&self) -> bool {
        self.boring_moves_counter >= 15
    }

    pub fn get_winner(&self) -> Option<Winner> {
        let moves = self.get_moves();
        if moves.is_empty() {
            return Some(if self.current_player {
                Winner::Black
            } else {
                Winner::White
            });
        }
        if self.is_draw() {
            return Some(Winner::Draw);
        }
        None
    }

    fn make_pawn_move(&mut self, from: i8, to: i8) {
        let from_mask = 1 << from;
        let to_mask = 1 << to;
        let is_white_from_bit = self.is_white & from_mask;
        let is_queen_from_bit = self.is_queen & from_mask;
        // Copy data to the new cell and clean the old cell
        self.not_empty ^= from_mask ^ to_mask;
        self.is_white ^= (is_white_from_bit >> from << to) ^ is_white_from_bit;
        self.is_queen ^= (is_queen_from_bit >> from << to) ^ is_queen_from_bit;
        self.eval_white += ((to - from) * (is_queen_from_bit == 0) as i8) as i32;
        // self.eval_white += (to - from) as i32;
        // Promotion to queen
        if (to > 55 && self.current_player || to < 8 && !self.current_player) && is_queen_from_bit == 0 {
            self.is_queen ^= to_mask;
            let player_coeff = ((self.current_player as i32) << 1) - 1;
            self.eval_white += player_coeff * (QUEEN_COST - PAWN_COST);
            self.eval_white += 63 * (!self.current_player) as i32 - to as i32;
        }
    }

    fn make_cutting_move(&mut self, from: i8, to: i8) {
        assert_eq!(get_bit(self.not_empty, from), 1);
        assert_eq!(get_bit(self.not_empty, to), 0);
        let mut curr_cell = from;
        let diff = to - from;
        let delta = diff.signum() * if diff % 7 == 0 {
            7
        } else {
            assert_eq!(diff % 9, 0);
            9
        };
        let mut it = 0;
        let captured_cell = loop {
            curr_cell += delta;
            if get_bit(self.not_empty, curr_cell) == 1 {
                break curr_cell;
            }
            it += 1;
            if it > 10 {
                panic!("Incorrect move: {from} {to}");
            }
        };
        let curr_player_add_eval = if get_bit(self.is_queen, captured_cell) == 1 {
            QUEEN_COST
        } else {
            self.eval_white += 63 * self.current_player as i32 - captured_cell as i32;
            PAWN_COST
        };
        let player_coeff = ((self.current_player as i32) << 1) - 1;
        self.eval_white += player_coeff * curr_player_add_eval;
        self.make_pawn_move(from, to);
        // Clear the cell with the captured checker
        let mask = !(1 << captured_cell);
        self.is_white &= mask;
        self.not_empty &= mask;
        self.is_queen &= mask;
    }

    pub fn make_move(&mut self, move_to_make: Move) {
        match move_to_make {
            Move::Simple(a, b) => {
                if get_bit(self.is_queen, a) == 1 {
                    self.boring_moves_counter += 1;
                } else {
                    self.boring_moves_counter = 0;
                }
                self.make_pawn_move(a, b)
            },
            Move::Take(v) => {
                self.boring_moves_counter = 0;
                for i in 1..v.len() {
                    self.make_cutting_move(v[i - 1], v[i]);
                }
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

    pub fn get_takes_from_cell_rev(&self, from: i8) -> Vec<Vec<i8>> {
        let enemy = self.not_empty
            & if self.current_player {
                !self.is_white
            } else {
                self.is_white
            };
        let cell_mask = 1 << from;
        let mut moves = Vec::new();
        if self.is_queen & cell_mask == 0 {
            if self.get_pawns_left_up_takes_mask(cell_mask, enemy) != 0 {
                moves.push(from + 18);
            }
            if self.get_pawns_right_up_takes_mask(cell_mask, enemy) != 0 {
                moves.push(from + 14);
            }
            if self.get_pawns_left_down_takes_mask(cell_mask, enemy) != 0 {
                moves.push(from - 14);
            }
            if self.get_pawns_right_down_takes_mask(cell_mask, enemy) != 0 {
                moves.push(from - 18);
            }
            // let me = !enemy & self.not_empty;
            // let diagonal = DIAGONALS[from as usize];
            // let magic_num = MAGIC_NUMBERS[from as usize];
            // let blockers: u64 = me & diagonal;
            // let magic_i = ((blockers.wrapping_mul(magic_num)) >> MAGIC_RSHIFT) as usize;
            // let before_blocker = MOVES_WITHOUT_CAPTURES[MAX_POSITION_MAGIC_INDEX * from as usize + magic_i];
            // let cells_to_capture: u64 = enemy & diagonal;
            // let magic_i = ((cells_to_capture.wrapping_mul(magic_num)) >> MAGIC_RSHIFT) as usize;
            // let after_capture = MOVES_WITH_CAPTURES[MAX_POSITION_MAGIC_INDEX * from as usize + magic_i];
            // assert_ne!(before_blocker, 1);
            // assert_ne!(after_capture, 1);
            // let mut can_move_to = (before_blocker & after_capture) & DIAGONALS_LIMITED_BY_2[from as usize];
            // while can_move_to != 0 {
            //     let mask = last_bit(can_move_to);
            //     can_move_to ^= mask;
            //     let j = get_bit_i(mask);
            //     moves.push(j);
            // }
        } else {
            let me = !enemy & self.not_empty;
            let diagonal = DIAGONALS[from as usize];
            let magic_num = MAGIC_NUMBERS[from as usize];
            let blockers: u64 = me & diagonal;
            let magic_i = ((blockers.wrapping_mul(magic_num)) >> MAGIC_RSHIFT) as usize;
            let before_blocker = MOVES_WITHOUT_CAPTURES[MAX_POSITION_MAGIC_INDEX * from as usize + magic_i];
            let cells_to_capture: u64 = enemy & diagonal;
            let magic_i = ((cells_to_capture.wrapping_mul(magic_num)) >> MAGIC_RSHIFT) as usize;
            let after_capture = MOVES_WITH_CAPTURES[MAX_POSITION_MAGIC_INDEX * from as usize + magic_i];
            assert_ne!(before_blocker, 1);
            assert_ne!(after_capture, 1);
            let mut can_move_to = before_blocker & after_capture;
            while can_move_to != 0 {
                let mask = last_bit(can_move_to);
                can_move_to ^= mask;
                let j = get_bit_i(mask);
                moves.push(j);
            }
        }
        let mut new_moves: Vec<Vec<i8>> = Vec::new();
        for to in moves {
            let mut game_copy = self.clone();
            // dbg!(&self);
            // println!("{from} {to}");
            game_copy.make_cutting_move(from, to);
            // dbg!(&self);
            let part2 = game_copy.get_takes_from_cell_rev(to);
            if part2.is_empty() {
                new_moves.push(vec![to, from]);
            } else {
                new_moves.extend(part2.into_iter().map(|mut m| {
                    m.push(from);
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
                let from = i;
                let to = i + 2 * add;
                let mut game_copy = self.clone();
                game_copy.make_cutting_move(from, to);
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
        let my_pieces = if self.current_player {
            self.is_white & self.not_empty
        } else {
            !self.is_white & self.not_empty
        };
        let my_pawns = !self.is_queen & my_pieces;
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
        let mut my_queens = my_pieces & self.is_queen;
        while my_queens != 0 {
            let curr_queen_mask = last_bit(my_queens);
            let coord = get_bit_i(curr_queen_mask);
            my_queens ^= curr_queen_mask;
            let blockers: u64 = self.not_empty & DIAGONALS[coord as usize];
            let magic_i = ((blockers.wrapping_mul(MAGIC_NUMBERS[coord as usize])) >> MAGIC_RSHIFT) as usize;
            let mut can_move_to = MOVES_WITHOUT_CAPTURES[MAX_POSITION_MAGIC_INDEX * coord as usize + magic_i];
            assert_ne!(can_move_to, 1, "{coord} {blockers:b} {magic_i} {can_move_to:b}");
            while can_move_to != 0 {
                let mask = last_bit(can_move_to);
                can_move_to ^= mask;
                let j = get_bit_i(mask);
                moves.push(Move::Simple(coord, j));
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
    fn eq(&self, other: &Self) -> bool {
        let res = (self.not_empty, self.is_white, self.is_queen, self.current_player, self.boring_moves_counter) == (other.not_empty, other.is_white, other.is_queen, other.current_player, self.boring_moves_counter);
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

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
