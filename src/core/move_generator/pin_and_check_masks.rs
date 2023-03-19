use super::*;
use const_for::*;

/// Use king_sq * 64 + slider_sq
pub const SLIDER_HV_CHECK_MASK: [u64; 4096] = generate_hv_slider_check_mask();
/// Use king_sq * 64 + slider_sq
pub const SLIDER_D12_CHECK_MASK: [u64; 4096] = generate_d12_slider_check_mask();

/// This gets a pin mask between the king and the sliding piece if relevant
/// Use (king_sq * 2048) + (slider_sq * 256) + (pexed occupancies along the axis wanted)
pub const PIN_MASKS: [u64; 16384] = generate_pin_masks();

/// A const implementation of pext, used to access sliding attacks on compile time
const fn const_pext(value: u64, mut mask: u64) -> u64 {
    let mut res = 0;
    let mut bb: u64 = 1;
    while mask != 0 {
        if value & mask & (mask.wrapping_neg()) != 0 {
            res |= bb;
        }
        mask &= mask - 1;
        bb = bb.wrapping_add(bb);
    }
    res
}

const fn generate_hv_slider_check_mask() -> [u64; 4096] {
    const fn const_hv_attacks(square: u8, occ: u64) -> u64 {
        let offset = HV_ATTACK_OFFSETS[square as usize] as u64;
        let mask = HV_ATTACK_TABLE_MASKS[square as usize];
        let index = const_pext(occ, mask);
    
        HV_SLIDING_ATTACKS[(offset + index) as usize]
    }

    let mut masks: [u64; 4096] = [0; 4096];

    const_for!(square in 0..64 => {
        const_for!(king_pos in 0..64 => {
            let sq_hv_rays = const_hv_attacks(square as u8, 1 << king_pos);

            masks[king_pos * 64 + square] = {
               if RANK_MASKS[king_pos] == RANK_MASKS[square] || FILE_MASKS[king_pos] == FILE_MASKS[square] {
                    // HV
                    let king_hv_rays = const_hv_attacks(king_pos as u8, 1 << square);
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
    const fn const_d12_attacks(square: u8, occ: u64) -> u64 {
        let offset = D12_ATTACK_OFFSETS[square as usize] as u64;
        let mask = D12_ATTACK_TABLE_MASKS[square as usize];
        let index = const_pext(occ, mask);

        D12_SLIDING_ATTACKS[(offset + index) as usize]
    }

    let mut masks: [u64; 4096] = [0; 4096];

    const_for!(square in 0..64 => {
        const_for!(king_pos in 0..64 => {
            let sq_diag_rays = const_d12_attacks(square as u8, 1 << king_pos);

            masks[king_pos * 64 + square] = {
                if D1_MASKS[king_pos] == D1_MASKS[square] || D2_MASKS[king_pos] == D2_MASKS[square] {
                    // Diagonals
                    let king_diag_rays = const_d12_attacks(king_pos as u8, 1 << square);
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