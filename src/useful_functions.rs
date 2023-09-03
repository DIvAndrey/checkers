#[inline(always)]
pub fn get_bit(x: u64, i: i8) -> u64 {
    (x >> i) & 1
}

#[inline(always)]
pub fn after_bit(mask: u64) -> u64 {
    mask - 1
}

#[inline(always)]
pub fn before_bit(mask: u64) -> u64 {
    !(mask ^ (mask - 1))
}

#[inline(always)]
pub fn last_bit(x: u64) -> u64 {
    x & x.wrapping_neg()
}

#[inline(always)]
pub fn first_bit(x: u64) -> u64 {
    assert_ne!(x, 0);
    0x8000_0000_0000_0000u64.wrapping_shr(x.leading_zeros())
}

#[inline(always)]
pub fn first_bit_or_0(x: u64) -> u64 {
    // last_bit(x.reverse_bits()).reverse_bits()
    x & 0x8000_0000_0000_0000u64.wrapping_shr(x.leading_zeros())
}

#[inline(always)]
pub fn get_bit_i(mask: u64) -> i8 {
    if mask == 0 {
        return 0;
    }
    mask.trailing_zeros() as i8
}

#[inline(always)]
pub fn get_bit_i_or(mask: u64, val: i8) -> i8 {
    if mask == 0 {
        return val;
    }
    return get_bit_i(mask);
}

#[inline(always)]
pub fn conv_1d_to_2d(i: i8) -> (usize, usize) {
    assert!(i >= 0, "i = {}", i);
    assert!(i < 64, "i = {}", i);
    ((7 - (i & 0b111)) as usize, (7 - i / 8) as usize)
}

#[inline(always)]
pub fn conv_2d_to_1d(x: usize, y: usize) -> i8 {
    let i = 63 - x as i8 - 8 * y as i8;
    assert!(i >= 0, "i = {}", i);
    assert!(i < 64, "i = {}", i);
    i
}


pub const EXCLUDE_RIGHT_COLUMN: u64 = 0b_1111_1110___1111_1110___1111_1110___1111_1110___1111_1110___1111_1110___1111_1110___1111_1110;
pub const EXCLUDE_LEFT_COLUMN: u64 =  0b_0111_1111___0111_1111___0111_1111___0111_1111___0111_1111___0111_1111___0111_1111___0111_1111;
pub const EXCLUDE_2_RIGHT_COLUMNS: u64 = 0b_1111_1100___1111_1100___1111_1100___1111_1100___1111_1100___1111_1100___1111_1100___1111_1100;
pub const EXCLUDE_2_LEFT_COLUMNS: u64 =  0b_0011_1111___0011_1111___0011_1111___0011_1111___0011_1111___0011_1111___0011_1111___0011_1111;

// These arrays contain numbers whose bits correspond to diagonals on the game board.
// It is an easy way to get diagonals containing the given cell.
pub const DIAGONALS_UP_RIGHT: [u64; 64] = [
    1,
    258,
    66052,
    16909320,
    4328785936,
    1108169199648,
    283691315109952,
    72624976668147840,
    258,
    66052,
    16909320,
    4328785936,
    1108169199648,
    283691315109952,
    72624976668147840,
    145249953336295424,
    66052,
    16909320,
    4328785936,
    1108169199648,
    283691315109952,
    72624976668147840,
    145249953336295424,
    290499906672525312,
    16909320,
    4328785936,
    1108169199648,
    283691315109952,
    72624976668147840,
    145249953336295424,
    290499906672525312,
    580999813328273408,
    4328785936,
    1108169199648,
    283691315109952,
    72624976668147840,
    145249953336295424,
    290499906672525312,
    580999813328273408,
    1161999622361579520,
    1108169199648,
    283691315109952,
    72624976668147840,
    145249953336295424,
    290499906672525312,
    580999813328273408,
    1161999622361579520,
    2323998145211531264,
    283691315109952,
    72624976668147840,
    145249953336295424,
    290499906672525312,
    580999813328273408,
    1161999622361579520,
    2323998145211531264,
    4647714815446351872,
    72624976668147840,
    145249953336295424,
    290499906672525312,
    580999813328273408,
    1161999622361579520,
    2323998145211531264,
    4647714815446351872,
    9223372036854775808,
];
pub const DIAGONALS_UP_LEFT: [u64; 64] = [
    9241421688590303745,
    36099303471055874,
    141012904183812,
    550831656968,
    2151686160,
    8405024,
    32832,
    128,
    4620710844295151872,
    9241421688590303745,
    36099303471055874,
    141012904183812,
    550831656968,
    2151686160,
    8405024,
    32832,
    2310355422147575808,
    4620710844295151872,
    9241421688590303745,
    36099303471055874,
    141012904183812,
    550831656968,
    2151686160,
    8405024,
    1155177711073755136,
    2310355422147575808,
    4620710844295151872,
    9241421688590303745,
    36099303471055874,
    141012904183812,
    550831656968,
    2151686160,
    577588855528488960,
    1155177711073755136,
    2310355422147575808,
    4620710844295151872,
    9241421688590303745,
    36099303471055874,
    141012904183812,
    550831656968,
    288794425616760832,
    577588855528488960,
    1155177711073755136,
    2310355422147575808,
    4620710844295151872,
    9241421688590303745,
    36099303471055874,
    141012904183812,
    144396663052566528,
    288794425616760832,
    577588855528488960,
    1155177711073755136,
    2310355422147575808,
    4620710844295151872,
    9241421688590303745,
    36099303471055874,
    72057594037927936,
    144396663052566528,
    288794425616760832,
    577588855528488960,
    1155177711073755136,
    2310355422147575808,
    4620710844295151872,
    9241421688590303745,
];
