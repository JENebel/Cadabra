use std::{env, path::Path, fs::{self, File}, io::Write};

pub const ROOK_MASK: [u64; 64] = generate_rook_masks();
pub const ROOK_ATTACK_MASK: [u64; 64] = generate_rook_attack_masks();
pub const BISHOP_MASK: [u64; 64] = generate_bishop_masks();
pub const BISHOP_ATTACK_MASK: [u64; 64] = generate_bishop_attack_masks();

/// From upper left to lower right
//pub const DIAG1_MASKS: [u64; 64] = generate_d1_masks();
/// From upper right to lower left
//pub const DIAG2_MASKS: [u64; 64] = generate_d1_masks();

fn main() {
    // Find file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("consts.rs");

    // Clear file
    File::create(&dest_path).expect("Couldn't clear consts.rs");

    // Open file
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&dest_path)
        .unwrap();

    // Write to file

    // Files/Ranks/Diagonals
    write!(file, "{}", array_string(generate_file_masks().to_vec(), "u64", "FILE_MASKS")).expect("Couldnt write FILE_MASKS!");
    let rank_masks = generate_rank_masks();
    write!(file, "{}", array_string(rank_masks.to_vec(), "u64", "RANK_MASKS")).expect("Couldnt write RANK_MASKS!");
    write!(file, "{}", array_string(generate_d1_masks().to_vec(), "u64", "D1_MASKS")).expect("Couldnt write D1_MASKS!");
    write!(file, "{}", array_string(generate_d2_masks().to_vec(), "u64", "D2_MASKS")).expect("Couldnt write D2_MASKS!");

    // Special ranks
    write!(file, "{}", format!("pub const END_RANKS_MASK: u64 = {};\n", rank_masks[0] | rank_masks[63])).expect("Couldnt write END_RANKS_MASK!");
    write!(file, "{}", format!("pub const PAWN_INIT_RANKS_MASK: u64 = {};\n\n", rank_masks[8] | rank_masks[55])).expect("Couldnt write PAWN_INIT_RANKS_MASK!");

    // Check and pin masks
    write!(file, "{}", array_string(generate_check_path().to_vec(), "u64", "SLIDER_CHECK_MASK")).expect("Couldnt write SLIDER_CHECK_MASK!");
    write!(file, "{}", array_string(generate_pin_masks().to_vec(), "u64", "PIN_MASKS")).expect("Couldnt write PIN_MASKS!");

    // Castling masks
    write!(file, "{}", array_string(generate_attacked_castling_masks().to_vec(), "u64", "ATTACKED_CASTLING_MASKS")).expect("Couldnt write ATTACKED_CASTLING_MASKS!");
    write!(file, "{}", array_string(generate_open_castling_masks().to_vec(), "u64", "OPEN_CASTLING_MASKS")).expect("Couldnt write OPEN_CASTLING_MASKS!");

    // Pawns
    write!(file, "{}", array_string(generate_pawn_attacks(true).to_vec(), "u64", "WHITE_PAWN_ATTACKS")).expect("Couldnt write WHITE_PAWN_ATTACKS!");
    write!(file, "{}", array_string(generate_pawn_attacks(false).to_vec(), "u64", "BLACK_PAWN_ATTACKS")).expect("Couldnt write BLACK_PAWN_ATTACKS!");

    // Leapers
    write!(file, "{}", array_string(generate_knight_attacks().to_vec(), "u64", "KNIGHT_ATTACKS")).expect("Couldnt write KNIGHT_ATTACKS!");
    write!(file, "{}", array_string(generate_king_attacks().to_vec(), "u64", "KING_ATTACKS")).expect("Couldnt write KING_ATTACKS!");

    // Sliding pieces
    write!(file, "{}", array_string(ROOK_MASK.to_vec(), "u64", "ROOK_MASK")).expect("Couldnt write ROOK_MASK!");
    write!(file, "{}", array_string(BISHOP_MASK.to_vec(), "u64", "BISHOP_MASK")).expect("Couldnt write BISHOP_MASK!");

    let (rook_offsets, bishop_offsets, attacks) = generate_sliding_attacks();
    write!(file, "{}", array_string(rook_offsets.to_vec(), "usize", "ROOK_OFFSETS")).expect("Couldnt write ROOK_OFFSETS!");
    write!(file, "{}", array_string(bishop_offsets.to_vec(), "usize", "BISHOP_OFFSETS")).expect("Couldnt write BISHOP_OFFSETS!");
    write!(file, "{}", array_string(attacks.to_vec(), "u64", "SLIDING_ATTACKS")).expect("Couldnt write SLIDING_ATTACKS!");
}

fn array_string(data: Vec<u64>, type_string: &str, cons_name: &str) -> String {
    let len = data.len();
    let mut result = (if len < 1000 { "pub const "} else { "pub static " } ).to_string();
    result += cons_name;
    result += &format!(": [{}; {}] = [", type_string, len).to_string();

    let line_width = (len as f64).sqrt() as usize;
    for i in 0..len {
        if i % line_width == 0 { result += "\n" }
        result += &format!("{}{}", data[i], if i == len-1 {""} else {", "}).to_string();
    }
    result += "\n];\n\n";

    result
}

fn generate_open_castling_masks() -> [u64; 4] {
    let mut masks = [0; 4];

    // White
    masks[0] |= 1 << 62 | 1 << 61;
    masks[1] |= 1 << 59 | 1 << 58 | 1 << 57;

    // Black
    masks[2] |= 1 << 5 | 1 << 6;
    masks[3] |= 1 << 1 | 1 << 2 | 1 << 3;
    masks
}

fn generate_attacked_castling_masks() -> [u64; 4] {
    let mut masks = [0; 4];

    // White
    masks[0] |= 1 << 62 | 1 << 61;
    masks[1] |= 1 << 59 | 1 << 58;

    // Black
    masks[2] |= 1 << 5 | 1 << 6;
    masks[3] |= 1 << 2 | 1 << 3;
    masks
}

fn generate_file_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    for rank in 0..8 {
        for file in 0..8 {
            for i in 0..8 {
                masks[rank * 8 + file] |= (1 << file) << i*8;
            }
        }
    }
    masks
}

fn generate_rank_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    for rank in 0..8 {
        for file in 0..8 {
            for i in 0..8 {
                masks[rank * 8 + file] |= (1 << i) << 8*rank;
            }
        }
    }
    masks
}

fn generate_d1_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    for rank in 0..8 {
        for file in 0..8 {
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
        }
    }

    masks
}

fn generate_d2_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    
    for rank in 0..8 {
        for file in 0..8 {
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
        }
    }

    masks
}

fn generate_sliding_attacks() -> ([u64; 64], [u64; 64], Box<[u64; 107648]>) {
    //Sliding pieces
    let mut rook_offsets: [u64; 64] = [0; 64];
    let mut bishop_offsets: [u64; 64] = [0; 64];
    let mut attacks: Box<[u64; 107648]> = Box::new([0; 107648]);
    {
        let mut current_offset: u32 = 0;

        //ROOKS
        for rank in 0..8 {
            let mut file: u8 = 0;
            while file < 8 {
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
                
                file += 1;
            }
        }
        //OFFSET HER: 104600 i believe
        //Bishops
        let mut rank: u8 = 0;
        while rank < 8 {
            let mut file: u8 = 0;
            while file < 8 {
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
                
                file += 1;
            }
        rank += 1;
        }
    }

    (rook_offsets, bishop_offsets, attacks)
}

fn generate_pawn_attacks(color: bool) -> [u64; 64] {
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

fn generate_knight_attacks() -> [u64; 64] {
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

fn generate_king_attacks() -> [u64; 64] {
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

fn generate_check_path() -> [u64; 4096] {
    let mut masks: [u64; 4096] = [0; 4096];
    let ranks = generate_rank_masks();
    let files = generate_file_masks();
    let d1s = generate_d1_masks();
    let d2s = generate_d2_masks();

    for square in 0..64 {
        for king_pos in 0..64 {
            let sq_hv_rays = rook_attacks_on_the_fly(square as u8, 1 << king_pos);
            let sq_diag_rays = bishop_attacks_on_the_fly(square as u8, 1 << king_pos);

            masks[king_pos * 64 + square] = {
               if ranks[king_pos] == ranks[square] || files[king_pos] == files[square] {
                    // HV
                    let king_hv_rays = rook_attacks_on_the_fly(king_pos as u8, 1 << square);
                    king_hv_rays & sq_hv_rays | 1 << square
                } else if d1s[king_pos] == d1s[square] || d2s[king_pos] == d2s[square] {
                    // Diagonals
                    let king_diag_rays = bishop_attacks_on_the_fly(king_pos as u8, 1 << square);
                    king_diag_rays & sq_diag_rays | 1 << square
                } else {
                    u64::MAX
                }
            }
        }
    }
    
    masks
}

fn generate_pin_masks() -> Box<[u64; 16384]> {
    let mut masks: Box<[u64; 16384]> = Box::new([0; 16384]);
    
    for king_pos in 0..8 {
        for square in 0..8 {
            for occ in 0..256 {
                let mut mask = 0; 

                let mut found_either: bool = false;
                let mut between = 0;

                for i in 0..8 {
                    if i == king_pos {
                        println!("ki: {i}");
                        // Found king
                        if found_either {
                            break;
                        }
            
                        found_either = true;
                    } else if i == square {
                        println!("sl: {i}");
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
                }

                if between == 1 {
                    masks[2048*king_pos   +   256*square   +    occ] = mask
                }
            }
        }
    }
    
    masks
}

const fn generate_rook_masks() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = rook_mask(rank*8+file);
            file += 1;
            index += 1;
        }
        rank += 1;
    }
    mask
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

const fn generate_bishop_masks() -> [u64; 64] {
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

const fn set_occupancy(index: u32, attack_mask: u64) -> u64 {
    let mut occ = 0;

    let mut mask = attack_mask;

    let bits_in_mask = attack_mask.count_ones();
    let mut count: u16 = 0;
    let mut square;
    while count < bits_in_mask as u16 {
        //least significant 1 bit
        square = mask.trailing_zeros();

        //unset the bit
        mask ^= 1 << square;

        if (index & (1 << count)) != 0 {
            occ |= 1 << (square);
        }

        count += 1;
    }
    occ
}