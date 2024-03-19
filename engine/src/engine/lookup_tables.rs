use const_for::*;

pub const RANK_MASKS: [u64; 64] = generate_rank_masks();
pub const FILE_MASKS: [u64; 64] = generate_file_masks();
pub const D1_MASKS: [u64; 64] = generate_d1_masks();
pub const D2_MASKS: [u64; 64] = generate_d2_masks();

pub const TOP_RANK: u64 = RANK_MASKS[0];
pub const BOTTOM_RANK: u64 = RANK_MASKS[63];
pub const END_RANKS: u64 = TOP_RANK | BOTTOM_RANK;

pub const PAWN_INIT_WHITE_RANK: u64 = RANK_MASKS[55];
pub const PAWN_INIT_BLACK_RANK: u64 = RANK_MASKS[8];

pub const LOOKUP_RANK: [usize; 64] = [
    7, 7, 7, 7, 7, 7, 7, 7,
    6, 6, 6, 6, 6, 6, 6, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    4, 4, 4, 4, 4, 4, 4, 4,
    3, 3, 3, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
	0, 0, 0, 0, 0, 0, 0, 0,
];

pub const LOOKUP_FILE: [usize; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
	0, 1, 2, 3, 4, 5, 6, 7,
];

pub const LOOKUP_D1: [usize; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 0,
    2, 2, 2, 2, 2, 2, 1, 0,
    3, 3, 3, 3, 3, 2, 1, 0,
    4, 4, 4, 4, 3, 2, 1, 0,
    5, 5, 5, 4, 3, 2, 1, 0,
    6, 6, 5, 4, 3, 2, 1, 0,
	7, 6, 5, 4, 3, 2, 1, 0,
];

pub const LOOKUP_D2: [usize; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1,
    0, 1, 2, 2, 2, 2, 2, 2,
    0, 1, 2, 3, 3, 3, 3, 3,
    0, 1, 2, 3, 4, 4, 4, 4,
    0, 1, 2, 3, 4, 5, 5, 5,
    0, 1, 2, 3, 4, 5, 6, 6,
	0, 1, 2, 3, 4, 5, 6, 7,
];

const fn generate_rank_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            const_for!(i in 0..8 => {
                masks[rank * 8 + file] |= (1 << i) << (8*rank);
            })
        })
    });
    masks
}

const fn generate_file_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            const_for!(i in 0..8 => {
                masks[rank * 8 + file] |= (1 << file) << (i*8);
            });
        });
    });
    masks
}

const fn generate_d1_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            let mut r = rank;
            let mut f = file;
            while f < 7 && r < 7 {
                f += 1;
                r += 1;
                masks[rank * 8 + file] |= (1 << f) << (8*r);
            }
            r = rank;
            f = file;
            while f > 0 && r > 0 {
                f -= 1;
                r -= 1;
                masks[rank * 8 + file] |= (1 << f) << (8*r);
            }
            masks[rank * 8 + file] |= 1 << (8 * rank + file)
        });
    });

    masks
}

const fn generate_d2_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            masks[rank * 8 + file] |= (1 << file) << (8*rank);
            let mut r = rank;
            let mut f = file;
            while f < 7 && r > 0 {
                f += 1;
                r -= 1;
                masks[rank * 8 + file] |= (1 << f) << (8*r);
                
            }
            r = rank;
            f = file;
            while f > 0 && r < 7 {
                f -= 1;
                r += 1;
                masks[rank * 8 + file] |= (1 << f) << (8*r);
            }
            masks[rank * 8 + file] |= 1 << (8 * rank + file)
        });
    });

    masks
}