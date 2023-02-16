use super::*;

#[inline]
pub fn perft<const DETAILED: bool>(pos: &Position, depth: u8) -> u64 {
    let moves = pos.generate_moves();

    if depth == 1 {
        return moves.len() as u64
    }

    let mut nodes = 0;

    for m in moves {
        let mut copy = *pos;
        copy.make_move(m);

        let temp_res = perft::<false>(&copy, depth - 1);

        if DETAILED {
            println!("{}: {temp_res}", m.to_uci_string());
        }

        nodes += temp_res
    }

    nodes
}