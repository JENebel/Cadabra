use super::*;
use bitintr::Pext;
use const_for::*;

pub const RANK_MASKS: [u64; 64] = generate_rank_masks();
pub const FILE_MASKS: [u64; 64] = generate_file_masks();
pub const D1_MASKS: [u64; 64] = generate_d1_masks();
pub const D2_MASKS: [u64; 64] = generate_d2_masks();
pub const HV_RAYS: [u64; 64] = generate_hv_rays();
pub const D12_RAYS: [u64; 64] = generate_d12_rays();

pub const TOP_RANK: u64 = RANK_MASKS[0];
pub const BOTTOM_RANK: u64 = RANK_MASKS[63];
pub const END_RANKS: u64 = TOP_RANK | BOTTOM_RANK;

pub const PAWN_INIT_WHITE_RANK: u64 = RANK_MASKS[55];
pub const PAWN_INIT_BLACK_RANK: u64 = RANK_MASKS[8];
pub const PAWN_INIT_RANKS_MASKS: u64 = PAWN_INIT_WHITE_RANK | PAWN_INIT_BLACK_RANK;


/// Use king_sq * 64 + slider_sq
pub const SLIDER_HV_CHECK_MASK: [u64; 4096] = generate_hv_slider_check_mask();
/// Use king_sq * 64 + slider_sq
pub const SLIDER_D12_CHECK_MASK: [u64; 4096] = generate_d12_slider_check_mask();

/// This gets a pin mask between the king and the sliding piece if relevant
/// Use (king_sq * 2048) + (slider_sq * 256) + (pexed occupancies along the axis wanted)
pub const PIN_MASKS: [u64; 16384] = generate_pin_masks();

// Attack tables
pub const ROOK_MASK: [u64; 64] = generate_ROOK_MASK();
pub const BISHOP_MASK: [u64; 64] = generate_BISHOP_MASK();

lazy_static::lazy_static!(
    pub static ref SLIDING_ATTACKS: ([u64; 64], [u64; 64], Box<[u64; 107648]>) = generate_sliding_attacks();
);

pub const WHITE_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(true);
pub const BLACK_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(false);
pub const KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();
pub const KING_ATTACKS: [u64; 64] = generate_king_attacks();

#[inline(always)]
pub fn get_attacks(square: u8, color: Color, piece_type: PieceType, occupancies: Bitboard) -> u64 {
    match piece_type {
        PieceType::Pawn =>   pawn_attacks(square, color),
        PieceType::Knight => knight_attacks(square),
        PieceType::Bishop => d12_attacks(square, occupancies),
        PieceType::Rook =>   hv_attacks(square, occupancies),
        PieceType::Queen =>  d12_attacks(square, occupancies) | hv_attacks(square, occupancies),
        PieceType::King =>   king_attacks(square),
    }
}

/// Gets the possible pawn attacks from the current position
#[inline(always)]
pub fn pawn_attacks(square: u8, color: Color) -> u64 {
    if color.is_white() {
        WHITE_PAWN_ATTACKS[square as usize]
    }
    else {
        BLACK_PAWN_ATTACKS[square as usize]
    }
}

#[inline(always)]
pub fn knight_attacks(square: u8) -> u64 {
    KNIGHT_ATTACKS[square as usize]
}

#[inline(always)]
pub fn king_attacks(square: u8) -> u64 {
    KING_ATTACKS[square as usize]
}

#[inline(always)]
pub fn hv_attacks(square: u8, occ: Bitboard) -> u64 {
    let offset = SLIDING_ATTACKS.0[square as usize] as u64;
    let mask = ROOK_MASK[square as usize];
    let index = occ.as_u64().pext(mask);

    SLIDING_ATTACKS.2[(offset + index) as usize]
}

#[inline(always)]
pub fn d12_attacks(square: u8, occ: Bitboard) -> u64 {
    let offset = SLIDING_ATTACKS.1[square as usize] as u64;
    let mask = BISHOP_MASK[square as usize];
    let index = occ.as_u64().pext(mask);

    SLIDING_ATTACKS.2[(offset + index) as usize]
}

const fn generate_rank_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            const_for!(i in 0..8 => {
                masks[rank * 8 + file] |= (1 << i) << 8*rank;
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
                masks[rank * 8 + file] |= (1 << file) << i*8;
            });
        });
    });
    masks
}

const fn generate_hv_rays() -> [u64; 64] {
    let mut masks = [0; 64];
    let ranks = generate_rank_masks();
    let files = generate_file_masks();
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            masks[rank * 8 + file] |= ranks[rank * 8 + file];
            masks[rank * 8 + file] |= files[rank * 8 + file];
        });
    });
    masks
}

const fn generate_d12_rays() -> [u64; 64] {
    let mut masks = [0; 64];
    let d1 = generate_d1_masks();
    let d2 = generate_d2_masks();
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            masks[rank * 8 + file] |= d1[rank * 8 + file];
            masks[rank * 8 + file] |= d2[rank * 8 + file];
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
                masks[rank * 8 + file] |= (1 << f) << 8*r;
            }
            r = rank;
            f = file;
            while f > 0 && r > 0 {
                f -= 1;
                r -= 1;
                masks[rank * 8 + file] |= (1 << f) << 8*r;
            }
            masks[rank * 8 + file] |= 1 << 8 * rank + file
        });
    });

    masks
}

const fn generate_d2_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            masks[rank * 8 + file] |= (1 << file) << 8*rank;
            let mut r = rank;
            let mut f = file;
            while f < 7 && r > 0 {
                f += 1;
                r -= 1;
                masks[rank * 8 + file] |= (1 << f) << 8*r;
                
            }
            r = rank;
            f = file;
            while f > 0 && r < 7 {
                f -= 1;
                r += 1;
                masks[rank * 8 + file] |= (1 << f) << 8*r;
            }
            masks[rank * 8 + file] |= 1 << 8 * rank + file
        });
    });

    masks
}

const fn generate_ROOK_MASK() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    const_for!(rank in 0..8 => {
        const_for!(file in 0..8 => {
            mask[index] = rook_mask(rank*8+file);
            index += 1;
        })
    });
    
    mask
}

fn generate_sliding_attacks() -> ([u64; 64], [u64; 64], Box<[u64; 107648]>) {
    //Sliding pieces
    let mut rook_offsets: [u64; 64] = [0; 64];
    let mut bishop_offsets: [u64; 64] = [0; 64];
    let mut attacks: Box<[u64; 107648]> = Box::new([0; 107648]);
    {
        let mut current_offset: u32 = 0;

        //ROOKS
        const_for!(rank in 0..8 => {
            const_for!(file in 0..8 => {
                let square = rank * 8 + file;
                rook_offsets[square as usize] = current_offset as u64;
                let number_of_occupancies = (2 as u16).pow(ROOK_MASK[square as usize].count_ones()) as u32;

                let mut occ_index: u32 = 0;
                while occ_index < number_of_occupancies {
                    let occ = set_occupancy(occ_index, ROOK_MASK[square as usize]);
                    attacks[(current_offset + occ_index as u32) as usize] = rook_attacks_on_the_fly(square, occ);
                    occ_index += 1;
                }
                
                current_offset += number_of_occupancies as u32;
            });
        });
        //OFFSET HER: 102400 i believe
        println!("{current_offset}");
        //Bishops
        const_for!(rank in 0..8 => {
            const_for!(file in 0..8 => {
                let square = rank * 8 + file;
                bishop_offsets[square as usize] = current_offset as u64;
                let number_of_occupancies = (2 as u16).pow(BISHOP_MASK[square as usize].count_ones()) as u32;

                let mut occ_index: u32 = 0;
                while occ_index < number_of_occupancies {
                    let occ = set_occupancy(occ_index, BISHOP_MASK[square as usize]);
                    attacks[(current_offset + occ_index as u32) as usize] = bishop_attacks_on_the_fly(square, occ);
                    occ_index += 1;
                }
                
                current_offset += number_of_occupancies as u32;
            });
        });
        println!("{current_offset}");
    }

    (rook_offsets, bishop_offsets, attacks)
}

const fn generate_pawn_attacks(color: bool) -> [u64; 64] {
    let mut attacks = [0; 64];
    
    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if color {
                if file != 7 { result = result | base >> 7 as u64 }
                if file != 0 { result = result | base >> 9 as u64 }
                
            } else {
                if file != 0 { result = result | base << 7 as u64 }
                if file != 7 { result = result | base << 9 as u64 }
            }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_knight_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if rank > 1 && file < 7 { result = result | base >> 15 as u64 }
            if rank > 0 && file < 6 { result = result | base >> 6 as u64 }

            if rank < 7 && file < 6 { result = result | base << 10 as u64 }
            if rank < 6 && file < 7 { result = result | base << 17 as u64 }

            if rank > 1 && file > 0 { result = result | base >> 17 as u64 }
            if rank > 0 && file > 1 { result = result | base >> 10 as u64 }

            if rank < 7 && file > 1 { result = result | base << 6 as u64 }
            if rank < 6 && file > 0 { result = result | base << 15 as u64 }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_king_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if rank > 0 { result = result | base >> 8 as u64 }
            if file > 0 { result = result | base >> 1 as u64 }
            if rank < 7 { result = result | base << 8 as u64 }
            if file < 7 { result = result | base << 1 as u64 }

            if file > 0 && rank > 0 { result = result | base >> 9 as u64 }
            if file < 7 && rank > 0 { result = result | base >> 7 as u64 }
            if file > 0 && rank < 7 { result = result | base << 7 as u64 }
            if file < 7 && rank < 7 { result = result | base << 9 as u64 }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_hv_slider_check_mask() -> [u64; 4096] {
    let mut masks: [u64; 4096] = [0; 4096];
    let ranks = generate_rank_masks();
    let files = generate_file_masks();

    const_for!(square in 0..64 => {
        const_for!(king_pos in 0..64 => {
            let sq_hv_rays = rook_attacks_on_the_fly(square as u8, 1 << king_pos);

            masks[king_pos * 64 + square] = {
               if ranks[king_pos] == ranks[square] || files[king_pos] == files[square] {
                    // HV
                    let king_hv_rays = rook_attacks_on_the_fly(king_pos as u8, 1 << square);
                    king_hv_rays & sq_hv_rays | 1 << square
                } else {
                    u64::MAX
                }
            }
        });
    });
    
    masks
}

const fn generate_d12_slider_check_mask() -> [u64; 4096] {
    let mut masks: [u64; 4096] = [0; 4096];
    let d1s = generate_d1_masks();
    let d2s = generate_d2_masks();

    const_for!(square in 0..64 => {
        const_for!(king_pos in 0..64 => {
            let sq_diag_rays = bishop_attacks_on_the_fly(square as u8, 1 << king_pos);

            masks[king_pos * 64 + square] = {
                if d1s[king_pos] == d1s[square] || d2s[king_pos] == d2s[square] {
                    // Diagonals
                    let king_diag_rays = bishop_attacks_on_the_fly(king_pos as u8, 1 << square);
                    king_diag_rays & sq_diag_rays | 1 << square
                } else {
                    u64::MAX
                }
            }
        });
    });
    
    masks
}

const fn generate_pin_masks() -> [u64; 16384] {
    let mut masks: [u64; 16384] = [0; 16384];
    
    const_for!(king_pos in 0..8 => {
        const_for!(square in 0..8 => {
            const_for!(occ in 0..256 => {
                let mut mask = 0; 

                let mut found_either: bool = false;
                let mut between = 0;

                if occ & 1 << square == 0 {
                    continue
                }

                let start = if king_pos < square {
                    king_pos
                } else {
                    0
                };

                const_for!(i in start..8 => {
                    if i == king_pos {
                        // Found king
                        if found_either {
                            break;
                        }
            
                        found_either = true;
                    } else if i == square {
                        // Found slider
                        mask |= 1 << i;
                        
                        if found_either {
                            break;
                        }
                        
                        found_either = true;
                    } else if occ & 1 << i != 0 && found_either {
                        // Found piece between
                        mask |= 1 << i;
                        between += 1;
                    } else if found_either {
                        // Found empty between
                        mask |= 1 << i;
                    }
                });

                if between == 1 {
                    masks[2048*king_pos   +   256*square   +    occ] = mask
                }
            });
        });
    });
    
    masks
}

const fn set_occupancy(index: u32, attack_mask: u64) -> u64 {
    let mut occ = 0;

    let mut mask = attack_mask;

    const_for!(count in 0..attack_mask.count_ones() => {
        //least significant 1 bit
        let square = mask.trailing_zeros();

        //unset the bit
        mask ^= 1 << square;

        if (index & (1 << count)) != 0 {
            occ |= 1 << (square);
        }
    });

    occ
}

const fn generate_rook_attack_masks() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = rook_attacks_on_the_fly(rank*8+file, 0);
            file += 1;
            index += 1;
        }
        rank += 1;
    }
    mask
}

const fn generate_BISHOP_MASK() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = bishop_mask(rank*8+file);

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    mask
}

const fn generate_bishop_attack_masks() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = bishop_attacks_on_the_fly(rank*8+file, 0);
            file += 1;
            index += 1;
        }
        rank += 1;
    }
    mask
}

const fn rook_mask(square: u8) -> u64 {
    let base: u64 = 1 << (square);
    let mut result: u64 = 0;

    let mut file = square % 8;
    let mut offs = 1;
    while file > 1 {
        result |= base >> offs;
        offs += 1;
        file -= 1;
    }

    file = square % 8;
    offs = 1;
    while file < 6 {
        result |= base << offs;
        offs += 1;
        file += 1;
    }

    let mut rank = square / 8;
    let mut offs = 8;
    while rank > 1 {
        result |= base >> offs;
        offs += 8;
        rank -= 1;
    }

    rank = square / 8;
    offs = 8;
    while rank < 6 {
        result |= base << offs;
        offs += 8;
        rank += 1;
    }

    result
}

const fn bishop_mask(square: u8) -> u64 {
    let base: u64 = 1 << (square);
    let mut result: u64 = 0;
    let rank = square / 8;
    let file = square % 8;

    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    //Down-Right
    while t_rank < 6 && t_file < 6 {
        offs += 9;

        result |= base << offs;

        t_rank += 1;
        t_file += 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Down-Left
    while t_rank < 6 && t_file > 1 {
        offs += 7;

        result |= base << offs;

        t_rank += 1;
        t_file -= 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Up-Left
    while t_rank > 1 && t_file > 1 {
        offs += 9;

        result |= base >> offs;

        t_rank -= 1;
        t_file -= 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Up-Right
    while t_rank > 1 && t_file < 6 {
        offs += 7;

        result |= base >> offs;

        t_rank -= 1;
        t_file += 1;
    }

    result
}

const fn rook_attacks_on_the_fly(square: u8, occ: u64) -> u64 {
    let base: u64 = 1 << (square);
    let mut result: u64 = 0;

    let mut file = square % 8;
    let mut offs = 0;
    //LEFT
    while file > 0 {
        offs += 1;

        result |= base >> offs;

        if occ & base >> offs != 0 { break; }

        file -= 1;
    }

    file = square % 8;
    offs = 0;
    //RIGHT
    while file < 7 {
        offs += 1;

        result |= base << offs;

        if occ & base << offs != 0 { break; }

        file += 1;
    }

    let mut rank = square / 8;
    let mut offs = 0;
    //UP
    while rank > 0 {
        offs += 8;

        result |= base >> offs;

        if occ & base >> offs != 0 { break; }

        rank -= 1;
    }

    rank = square / 8;
    offs = 0;
    //DOWN
    while rank < 7 {
        offs += 8;

        result |= base << offs;

        if occ & base << offs != 0 { break; }

        rank += 1;
    }

    result
}

const fn bishop_attacks_on_the_fly(square: u8, occ: u64) -> u64 {
    let base: u64 = 1 << (square);
    let mut result: u64 = 0;
    let rank = square / 8;
    let file = square % 8;

    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    //Down-Right
    while t_rank < 7 && t_file < 7 {
        offs += 9;

        result |= base << offs;

        if occ & base << offs != 0 { break; }

        t_rank += 1;
        t_file += 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Down-Left
    while t_rank < 7 && t_file > 0 {
        offs += 7;

        result |= base << offs;

        if occ & base << offs != 0 { break; }

        t_rank += 1;
        t_file -= 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Up-Left
    while t_rank > 0 && t_file > 0 {
        offs += 9;

        result |= base >> offs;

        if occ & base >> offs != 0 { break; }

        t_rank -= 1;
        t_file -= 1;
    }

    t_rank = rank;
    t_file = file;
    offs = 0;
    //Up-Right
    while t_rank > 0 && t_file < 7 {
        offs += 7;

        result |= base >> offs;

        if occ & base >> offs != 0 { break; }

        t_rank -= 1;
        t_file += 1;
    }

    result
}