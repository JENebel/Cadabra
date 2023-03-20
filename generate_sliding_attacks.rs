use std::{env, path::Path, fs::{self, File}};

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

    use std::io::Write;

    let (hv_masks, hv_offsets, hv_attacks) = generate_hv_sliding_attacks();
    write!(file, "{}", array_string(hv_masks, "u64", "HV_ATTACK_TABLE_MASKS")?)?;
    write!(file, "{}", array_string(hv_offsets.to_vec(), "usize", "HV_ATTACK_OFFSETS")?)?;
    write!(file, "{}", array_string(hv_attacks.to_vec(), "u64", "HV_SLIDING_ATTACKS")?)?;

    let (d12_masks, d12_offsets, d12_attacks) = generate_d12_sliding_attacks();
    write!(file, "{}", array_string(d12_masks, "u64", "D12_ATTACK_TABLE_MASKS")?)?;
    write!(file, "{}", array_string(d12_offsets.to_vec(), "usize", "D12_ATTACK_OFFSETS")?)?;
    write!(file, "{}", array_string(d12_attacks.to_vec(), "u64", "D12_SLIDING_ATTACKS")?)?;

    println!("cargo:rerun-if-changed=sliding_attacks.rs");

    Ok(())
}

fn array_string(data: Vec<u64>, type_str: &str, cons_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::fmt::Write;

    let mut result = String::new();

    let len = data.len();
    write!(result, "pub const {cons_name}: [{type_str}; {len}] = [")?;

    for i in 0..len {
        if i % 200 == 0 { write!(result, "\n")? }
        write!(result, "{},", data[i])?;
    }
    write!(result, "\n];\n\n")?;

    Ok(result)
}

fn generate_hv_sliding_attacks() -> (Vec<u64>, [u64; 64], Box<[u64; 102400]>) {
    let mut attacks: Box<[u64; 102400]> = Box::new([0; 102400]);
    let mut offsets: [u64; 64] = [0; 64];
    let mut current_offset = 0;

    let masks: Vec<u64> = (0..64).map(|sq| hv_mask_from(sq)).collect();

    for square in 0..64 {
        offsets[square] = current_offset;
        let number_of_occupancies = (2 as u64).pow(masks[square].count_ones());

        for occ_index in 0..number_of_occupancies {
            let occ = set_occupancy(occ_index, masks[square]);
            attacks[(current_offset + occ_index) as usize] = hv_attacks_on_the_fly(square as u8, occ);
        }
        
        current_offset += number_of_occupancies;
    }

    (masks, offsets, attacks)
}

fn generate_d12_sliding_attacks() -> (Vec<u64>, [u64; 64], [u64; 5248]) {
    let mut attacks: [u64; 5248] = [0; 5248];
    let mut offsets: [u64; 64] = [0; 64];
    let mut current_offset = 0;

    let masks: Vec<u64> = (0..64).map(|sq| bishop_mask_from(sq)).collect();

    for square in 0..64 {
        offsets[square] = current_offset;
        let number_of_occupancies = (2 as u64).pow(masks[square].count_ones()) as u64;

        for occ_index in 0..number_of_occupancies {
            let occ = set_occupancy(occ_index, masks[square]);
            attacks[(current_offset + occ_index) as usize] = d12_attacks_on_the_fly(square as u8, occ);
        }
        
        current_offset += number_of_occupancies;
    }

    (masks, offsets, attacks)
}

/// Generates hv rays from square, excluding it self and the last square in a ray
fn hv_mask_from(square: u8) -> u64 {
    let mut result: u64 = 0;

    for file in 1..(square % 8) {
        result |= 1 << square - file
    }

    for file in 1..(7 - square % 8) {
        result |= 1 << square + file
    }

    for rank in 1..(square / 8) {
        result |= 1 << square - rank * 8
    }

    for rank in 1..(7 - square / 8) {
        result |= 1 << square + rank * 8
    }

    result
}

/// Generates d12 rays from square, excluding it self and the last square in a ray
fn bishop_mask_from(square: u8) -> u64 {
    let src: u64 = 1 << (square);
    let mut result: u64 = 0;
    let rank = square / 8;
    let file = square % 8;

    // Down-Right
    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    while t_rank < 6 && t_file < 6 {
        offs += 9;
        result |= src << offs;
        t_rank += 1;
        t_file += 1;
    }

    // Down-Left
    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    while t_rank < 6 && t_file > 1 {
        offs += 7;
        result |= src << offs;
        t_rank += 1;
        t_file -= 1;
    }

    // Up-Left
    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    while t_rank > 1 && t_file > 1 {
        offs += 9;
        result |= src >> offs;
        t_rank -= 1;
        t_file -= 1;
    }

    // Up-Right
    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    while t_rank > 1 && t_file < 6 {
        offs += 7;
        result |= src >> offs;
        t_rank -= 1;
        t_file += 1;
    }

    result
}

fn hv_attacks_on_the_fly(square: u8, occ: u64) -> u64 {
    let src: u64 = 1 << (square);
    let mut result: u64 = 0;

    // Left
    let mut file = square % 8;
    let mut offs = 0;
    while file > 0 {
        offs += 1;
        result |= src >> offs;
        if occ & src >> offs != 0 { break }
        file -= 1;
    }

    // Right
    let mut file = square % 8;
    let mut offs = 0;
    while file < 7 {
        offs += 1;
        result |= src << offs;
        if occ & src << offs != 0 { break }
        file += 1;
    }

    // Up
    let mut rank = square / 8;
    let mut offs = 0;
    while rank > 0 {
        offs += 8;
        result |= src >> offs;
        if occ & src >> offs != 0 { break }
        rank -= 1;
    }

    // Down
    let mut rank = square / 8;
    let mut offs = 0;
    while rank < 7 {
        offs += 8;
        result |= src << offs;
        if occ & src << offs != 0 { break }
        rank += 1;
    }

    result
}

fn d12_attacks_on_the_fly(square: u8, occ: u64) -> u64 {
    let src: u64 = 1 << (square);
    let mut result: u64 = 0;
    let rank = square / 8;
    let file = square % 8;

    // Down-Right
    /*for i in 1..7-rank.max(file) {
        if occ & src << i * 9 != 0 { break; }
        result |= src << i * 9;
    }*/
    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    while t_rank < 7 && t_file < 7 {
        offs += 9;
        result |= src << offs;
        if occ & src << offs != 0 { break; }
        t_rank += 1;
        t_file += 1;
    }

    // Down-Left
    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    while t_rank < 7 && t_file > 0 {
        offs += 7;
        result |= src << offs;
        if occ & src << offs != 0 { break; }
        t_rank += 1;
        t_file -= 1;
    }

    // Up-Left
    let mut t_rank = rank;
    let mut t_file = file;
    let mut offs = 0;
    while t_rank > 0 && t_file > 0 {
        offs += 9;
        result |= src >> offs;
        if occ & src >> offs != 0 { break; }
        t_rank -= 1;
        t_file -= 1;
    }

    // Up-Right
    t_rank = rank;
    t_file = file;
    offs = 0;
    while t_rank > 0 && t_file < 7 {
        offs += 7;
        result |= src >> offs;
        if occ & src >> offs != 0 { break; }
        t_rank -= 1;
        t_file += 1;
    }

    result
}

fn set_occupancy(index: u64, attack_mask: u64) -> u64 {
    let mut occ = 0;

    let mut mask = attack_mask;

    for count in 0..attack_mask.count_ones() {
        // Least significant 1 bit
        let square = mask.trailing_zeros();

        // Unset the bit
        mask ^= 1 << square;

        if (index & (1 << count)) != 0 {
            occ |= 1 << (square);
        }
    }

    occ
}