use std::{env, path::Path, fs::{self, File}, io::Write};
use const_for::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sliding_attacks.rs");

    // Clear file
    File::create(&dest_path).expect("Couldn't clear sliding_attacks.rs");

    // Open file
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&dest_path)
        .unwrap();

    // Write to file

    let (hv_masks, hv_offsets, hv_attacks) = generate_hv_sliding_attacks();
    write!(file, "{}", array_string(hv_masks.to_vec(), "const", "u64", "HV_ATTACK_TABLE_MASKS"))?;
    write!(file, "{}", array_string(hv_offsets.to_vec(), "const", "usize", "HV_ATTACK_OFFSETS"))?;
    write!(file, "{}", array_string(hv_attacks.to_vec(), "const", "u64", "HV_SLIDING_ATTACKS"))?;

    let (d12_masks, d12_offsets, d12_attacks) = generate_d12_sliding_attacks();
    write!(file, "{}", array_string(d12_masks.to_vec(), "const", "u64", "D12_ATTACK_TABLE_MASKS"))?;
    write!(file, "{}", array_string(d12_offsets.to_vec(), "const", "usize", "D12_ATTACK_OFFSETS"))?;
    write!(file, "{}", array_string(d12_attacks.to_vec(), "const", "u64", "D12_SLIDING_ATTACKS"))?;

    println!("cargo:rerun-if-changed=sliding_attacks.rs");

    Ok(())
}

fn array_string(data: Vec<u64>, const_or_static_str: &str, type_str: &str, cons_name: &str) -> String {
    let len = data.len();
    let mut result = String::new();
    result += cons_name;
    result += &format!(": [{type_str}; {len}] = [").to_string();

    let line_width = (len as f64).sqrt() as usize;
    for i in 0..len {
        if i % line_width == 0 { result += "\n" }
        result += &format!("{}{}", data[i], if i == len-1 {""} else {", "}).to_string();
    }
    result += "\n];\n\n";

    format!("pub {const_or_static_str} {result}")
}

fn generate_hv_sliding_attacks() -> ([u64; 64], [u64; 64], Box<[u64; 102400]>) {
    let mut attacks: Box<[u64; 102400]> = Box::new([0; 102400]);
    let mut offsets: [u64; 64] = [0; 64];
    let mut current_offset: u32 = 0;

    let masks = generate_rook_masks();

    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            offsets[square as usize] = current_offset as u64;
            let number_of_occupancies = (2 as u16).pow(masks[square as usize].count_ones()) as u32;

            let mut occ_index: u32 = 0;
            while occ_index < number_of_occupancies {
                let occ = set_occupancy(occ_index, masks[square as usize]);
                attacks[(current_offset + occ_index as u32) as usize] = hv_attacks_on_the_fly(square, occ);
                occ_index += 1;
            }
            
            current_offset += number_of_occupancies as u32;
        }
    }
    (masks, offsets, attacks)
}

fn generate_d12_sliding_attacks() -> ([u64; 64], [u64; 64], [u64; 5248]) {
    let mut attacks: [u64; 5248] = [0; 5248];
    let mut offsets: [u64; 64] = [0; 64];
    let mut current_offset: u32 = 0;

    let masks = generate_bishop_masks();

    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            offsets[square as usize] = current_offset as u64;
            let number_of_occupancies = (2 as u16).pow(masks[square as usize].count_ones()) as u32;

            let mut occ_index: u32 = 0;
            while occ_index < number_of_occupancies {
                let occ = set_occupancy(occ_index, masks[square as usize]);
                attacks[(current_offset + occ_index as u32) as usize] = d12_attacks_on_the_fly(square, occ);
                occ_index += 1;
            }
            
            current_offset += number_of_occupancies as u32;
        }
    }
    (masks, offsets, attacks)
}

fn generate_rook_masks() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = rook_mask_from(rank*8+file);
            file += 1;
            index += 1;
        }
        rank += 1;
    }
    mask
}

fn generate_bishop_masks() -> [u64; 64] {
    let mut mask = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            mask[index] = bishop_mask_from(rank*8+file);

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    mask
}

fn rook_mask_from(square: u8) -> u64 {
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

fn bishop_mask_from(square: u8) -> u64 {
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

fn hv_attacks_on_the_fly(square: u8, occ: u64) -> u64 {
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

fn d12_attacks_on_the_fly(square: u8, occ: u64) -> u64 {
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

fn set_occupancy(index: u32, attack_mask: u64) -> u64 {
    let mut occ = 0;

    let mut mask = attack_mask;

    let mut square;
    const_for!(count in 0..attack_mask.count_ones() => {
        //least significant 1 bit
        square = mask.trailing_zeros();

        //unset the bit
        mask ^= 1 << square;

        if (index & (1 << count)) != 0 {
            occ |= 1 << (square);
        }
    });

    occ
}