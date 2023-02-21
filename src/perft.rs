use super::*;

impl Position {
    #[inline(always)]
    pub fn perft<const ROOT: bool>(&self, depth: u8) -> u64 {
        let moves = self.generate_moves();
    
        let is_next_leaf = depth == 2;
    
        moves.fold(0, |acc, m| {
            let mut copy = *self;
            copy.make_move(m);
    
            let sub_nodes = match is_next_leaf {
                true =>  copy.generate_moves().len() as u64,
                false => copy.perft::<false>(depth - 1)
            };
    
            if ROOT {
                println!("{}: {sub_nodes}", m.to_uci_string());
            }
    
            acc + sub_nodes
        })
    }
}