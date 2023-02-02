pub const PIECE_KEYS: [[u64; 64]; 12] = generate_piece_keys();
pub const ENPASSANT_KEYS: [u64; 64] = generate_enpassant_keys();
pub const CASTLE_KEYS: [u64; 16] = generate_castle_keys();
pub const SIDE_KEY: u64 = get_random_u64_number(4084590338).0;

pub const fn generate_castle_keys() -> [u64; 16] {
    let mut keys = [0; 16];

    let mut i = 0;
    let mut state = 3667794840;

    while i < 16 {
        let res = get_random_u64_number(state);
        state = res.1;
        keys[i] = res.0;
        i+=1;
    }

    keys
}

pub const fn generate_enpassant_keys() -> [u64; 64] {
    let mut keys = [0; 64];

    let mut sq = 0;
    let mut state = 862131765;

    while sq < 64 {
        let res = get_random_u64_number(state);
        state = res.1;
        keys[sq] = res.0;
        sq+=1;
    }

    keys
}

pub const fn generate_piece_keys() -> [[u64; 64]; 12] {
    let mut keys = [[0; 64]; 12];

    let mut p  = 0;
    let mut sq;
    let mut state = 2828886037;

    while p < 12 {
        sq = 0;
        while sq < 64 {
            let res = get_random_u64_number(state);
            state = res.1;
            keys[p][sq] = res.0;
            sq+=1;
        }
        p+=1
    }

    keys
}

pub const fn get_random_u32_number(state: u32) -> u32{
    let mut num: u64 = state as u64;

    // XOR shift algorithm
    num ^= num << 13;
    num ^= num >> 17;
    num ^= num << 5;

    // return random number
    return num as u32;
}

// generate 64-bit pseudo legal numbers
pub const fn get_random_u64_number(state: u32) -> (u64, u32) {
    // define 4 random numbers
    let n1 = get_random_u32_number(state);
    let n2 = get_random_u32_number(n1);
    let n3 = get_random_u32_number(n2);
    let n4 = get_random_u32_number(n3);
    
    // return random number
    return (n1 as u64 | ((n2 as u64) << 16) | ((n3 as u64) << 32) | ((n4 as u64) << 48), n4);
}