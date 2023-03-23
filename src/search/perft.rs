use super::*;

impl Position {
    #[inline(always)]
    pub fn perft<const ROOT: bool>(&self, depth: u8) -> u64 {
        let moves = self.generate_moves();
    
        let is_next_leaf = depth == 2;
    
        moves.into_iter().fold(0, |acc, moov| {
            let mut copy = *self;
            copy.make_move(moov);
    
            let sub_nodes = match is_next_leaf {
                true =>  copy.generate_moves().len() as u64,
                false => copy.perft::<false>(depth - 1)
            };
    
            if ROOT {
                println!("{moov}: {sub_nodes}");
            }
    
            acc + sub_nodes
        })
    }
}