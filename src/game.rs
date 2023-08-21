use crate::useful_functions::*;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

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

pub type Move = Vec<(i8, i8)>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Game {
    // Bitmasks for game field. If cell is empty, all bits must be equal to 0.
    pub not_empty: u64,
    pub is_white: u64,
    pub is_queen: u64,
    // true if white, false if black
    pub current_player: bool,
}

impl Game {
    pub fn new() -> Game {
        // Game {
        //     not_empty: 6172835437156124074,
        //     is_white: 1236961215914,
        //     is_queen: 0,
        //     current_player: false,
        // }
        Game {
            not_empty: 0b_0101_0101___1010_1010___0101_0101___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
            is_white:  0b_0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
            is_queen:  0b_0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000,
            current_player: true,
        }
        // Game {
        //     not_empty: 0b_0000_0000___1000_0010___0100_0000___0000_0000___0000_0000___0000_0000___0001_0000___0000_0000,
        //     is_white:  0b_0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___0001_0000___0000_0000,
        //     is_queen:  0b_0000_0000___1000_0010___0100_0000___0000_0000___0000_0000___0000_0000___0001_0000___0000_0000,
        //     current_player: true,
        // }
        // Game {
        //     not_empty: 0b_0101_0101___1010_1010___0101_0101___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
        //     is_white:  0b_0000_0000___0000_0000___0000_0000___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
        //     is_queen:  0b_0101_0101___1010_1010___0101_0101___0000_0000___0000_0000___1010_1010___0101_0101___1010_1010,
        //     current_player: true,
        // }
    }

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
        self.not_empty ^= mask;
        self.is_white ^= self.is_white & mask;
        self.is_queen ^= self.is_queen & mask;
    }

    #[inline(always)]
    pub fn change_player(&mut self) {
        self.current_player = !self.current_player;
    }

    #[inline(always)]
    fn neighbours_count(&self, my: u64) -> i32 {
        (((my << 7) & EXCLUDE_LEFT_SIDE & my).count_ones()
            + ((my << 9) & EXCLUDE_RIGHT_SIDE & my).count_ones()
            + ((my >> 9) & EXCLUDE_LEFT_SIDE & my).count_ones()
            + ((my >> 7) & EXCLUDE_RIGHT_SIDE & my).count_ones()) as i32
    }

    #[inline(always)]
    fn get_forward_moves_score(&self, player: bool, mut my: u64) -> i32 {
        let mask = 0b1111_1111;
        if !player {
            my = my.reverse_bits();
        }
        let mut res = 0;
        for i in 0..8 {
            res += (i + 1) * ((my >> (i * 8)) & mask).count_ones();
        }
        res as i32
    }

    #[inline(always)]
    pub fn evaluate_for(&self, player: bool, my: u64, _enemy: u64) -> i32 {
        let mut res = (my.count_ones() * 100 + (my & self.is_queen).count_ones() * 200) as i32;
        res += self.neighbours_count(my);
        res += self.get_forward_moves_score(player, my);
        res
    }

    #[inline(always)]
    pub fn evaluate(&self) -> i32 {
        let (white, black) = (
            self.is_white & self.not_empty,
            !self.is_white & self.not_empty,
        );
        self.evaluate_for(true, white, black) - self.evaluate_for(false, black, white)
    }

    pub fn make_pawn_move(&mut self, from: i8, to: i8) {
        // Copy data to the new cell
        self.not_empty ^= 1 << to;
        self.is_white ^= get_bit(self.is_white, from) << to;
        let queen_bit = get_bit(self.is_queen, from);
        self.is_queen ^= queen_bit << to;
        // Clear current cell
        self.clear_cell(from);
        // If it is necessary to make a queen
        if queen_bit == 0 && (to > 55 && self.current_player || to < 8 && !self.current_player) {
            self.is_queen ^= 1 << to;
        }
    }

    #[inline(always)]
    pub fn make_cutting_move(&mut self, from: i8, to: i8, cut_i: i8) {
        self.make_pawn_move(from, to);
        // Clear the cell with felled checker
        self.clear_cell(cut_i);
    }

    #[inline(always)]
    pub fn make_move(&mut self, mov: (Move, bool)) {
        let (mov, is_cutting) = mov;
        if is_cutting {
            for i in 1..mov.len() {
                self.make_cutting_move(mov[i - 1].0, mov[i].0, mov[i].1);
            }
        } else {
            assert_eq!(mov.len(), 2);
            self.make_pawn_move(mov[0].0, mov[1].0);
        }
    }

    #[inline(always)]
    fn get_pawns(&self) -> u64 {
        !self.is_queen & self.not_empty
    }

    #[inline(always)]
    fn get_pawns_left_up_cuts_mask(&self, current: u64, enemy: u64) -> u64 {
        current & (enemy >> 9) & (!self.not_empty >> 18) & EXCLUDE_LEFT_SIDE2
    }

    #[inline(always)]
    fn get_pawns_right_up_cuts_mask(&self, current: u64, enemy: u64) -> u64 {
        current & (enemy >> 7) & (!self.not_empty >> 14) & EXCLUDE_RIGHT_SIDE2
    }

    #[inline(always)]
    fn get_pawns_left_down_cuts_mask(&self, current: u64, enemy: u64) -> u64 {
        current & (enemy << 7) & (!self.not_empty << 14) & EXCLUDE_LEFT_SIDE2
    }

    #[inline(always)]
    fn get_pawns_right_down_cuts_mask(&self, current: u64, enemy: u64) -> u64 {
        current & (enemy << 9) & (!self.not_empty << 18) & EXCLUDE_RIGHT_SIDE2
    }

    pub fn get_cuts_from_cell_rev(&self, i: i8) -> Vec<Move> {
        let enemy = self.not_empty
            & if self.current_player {
                !self.is_white
            } else {
                self.is_white
            };
        let cell_mask = 1 << i;
        let mut moves = Vec::new();
        if self.is_queen & cell_mask == 0 {
            if self.get_pawns_left_up_cuts_mask(cell_mask, enemy) != 0 {
                moves.push((i + 18, i + 9));
            }
            if self.get_pawns_right_up_cuts_mask(cell_mask, enemy) != 0 {
                moves.push((i + 14, i + 7));
            }
            if self.get_pawns_left_down_cuts_mask(cell_mask, enemy) != 0 {
                moves.push((i - 14, i - 7));
            }
            if self.get_pawns_right_down_cuts_mask(cell_mask, enemy) != 0 {
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
            let cut_cell_mask = last_bit(before & right_up_cut_mask & EXCLUDE_RIGHT_SIDE);
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
            let cut_cell_mask = first_bit_or_0(after & left_up_cut_mask & EXCLUDE_RIGHT_SIDE);
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
            let cut_cell_mask = first_bit_or_0(after & right_up_cut_mask & EXCLUDE_LEFT_SIDE);
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
            let cut_cell_mask = last_bit(before & left_up_cut_mask & EXCLUDE_LEFT_SIDE);
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
        let mut new_moves: Vec<Move> = Vec::new();
        for (to, cut_i) in moves {
            let mut game_copy = self.clone();
            game_copy.make_cutting_move(i, to, cut_i);
            let part2 = game_copy.get_cuts_from_cell_rev(to);
            if part2.is_empty() {
                new_moves.push(vec![(to, cut_i), (i, -1)]);
            } else {
                new_moves.extend(part2.into_iter().map(
                    |mut m| {
                        m.last_mut().unwrap().1 = cut_i;
                        m.push((i, -1));
                        m
                    },
                ));
            }
        }
        new_moves
    }

    #[inline(always)]
    pub fn get_cuts_from_cell(&self, i: i8) -> Vec<Move> {
        let mut res = self.get_cuts_from_cell_rev(i);
        res.iter_mut().for_each(|m| m.reverse());
        res
    }

    pub fn get_moves_with_cutting(&self) -> Vec<Move> {
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
                (enemy >> 7) & (empty_cells >> 14) & EXCLUDE_RIGHT_SIDE2 & my,
            ),
            (
                -9,
                (enemy << 9) & (empty_cells << 18) & EXCLUDE_RIGHT_SIDE2 & my,
            ),
            (
                9,
                (enemy >> 9) & (empty_cells >> 18) & EXCLUDE_LEFT_SIDE2 & my,
            ),
            (
                -7,
                (enemy << 7) & (empty_cells << 14) & EXCLUDE_LEFT_SIDE2 & my,
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
                let part2 = game_copy.get_cuts_from_cell_rev(i + 2 * add);
                if part2.is_empty() {
                    moves.push(vec![from, to]);
                } else {
                    part2.into_iter().for_each(|mut m| {
                        m.pop();
                        m.push(to);
                        m.push(from);
                        m.reverse();
                        moves.push(m);
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

    pub fn get_moves_without_cutting(&self) -> Vec<Move> {
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
                (7, (empty >> 7) & EXCLUDE_RIGHT_SIDE & my_pawns),
                (9, (empty >> 9) & EXCLUDE_LEFT_SIDE & my_pawns),
            ]
        } else {
            [
                (-7, (empty << 7) & EXCLUDE_LEFT_SIDE & my_pawns),
                (-9, (empty << 9) & EXCLUDE_RIGHT_SIDE & my_pawns),
            ]
        };
        let mut moves = Vec::new();
        for (add, mut mask) in masks {
            while mask != 0 {
                // i is current cell number
                let last_bit = last_bit(mask);
                let i = get_bit_i(last_bit);
                mask &= !last_bit;
                moves.push(vec![(i, -1), (i + add, -1)]);
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
                moves.push(vec![(i, -1), (j, -1)]);
            }
        }
        moves
    }

    // bool value means whether player can cut
    #[inline(always)]
    pub fn get_moves(&self) -> (Vec<Move>, bool) {
        let mut moves_with_cutting = self.get_moves_with_cutting();
        moves_with_cutting.sort();
        if !moves_with_cutting.is_empty() {
            return (moves_with_cutting, true);
        }
        (self.get_moves_without_cutting(), false)
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

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}\n{}",
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
                .join("\n")
        )
    }
}
