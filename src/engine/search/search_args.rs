use super::*;

/// The arguments provided in go command
#[derive(Copy, Clone)]
pub struct SearchArgs {
    pub max_depth: u8,
    pub time_target: u128,
    pub max_nodes: u128,
    pub ponder: bool,
}

const INF: u128 = 3155692597470; // 100 years in milliseconds, aka Infinite

impl SearchArgs {
    pub fn new_simple_depth(depth: u8) -> Self {
        Self::new(Some(depth), false, false, None, None, None, None, None).unwrap()
    }

    pub fn new(max_depth: Option<u8>, ponder: bool, infinite: bool, time_left: Option<u128>, inc: Option<u128>, movestogo: Option<u8>, nodes: Option<u128>, movetime: Option<u128>) -> Result<Self, String> {
        let time_target = if let Some(movetime) = movetime { // Fixed time search
            movetime - 250  // Buffer
        } else if infinite || max_depth.is_some() { // No time limit
            INF
        } else if let Some(time_left) = time_left { // Time control search
            let inc = inc.unwrap_or(0);
            let moves_to_go = movestogo.unwrap_or(30) as u128;

            if time_left < inc {
                (time_left + inc).max(250) - 250
            } else {
                ((time_left + (inc / 2)) / moves_to_go) + (inc / 2)
            }
        } else {
            unreachable!("No time control specified")
        };

        Ok(Self {
            max_depth: max_depth.unwrap_or(MAX_DEPTH),
            time_target,
            max_nodes: nodes.unwrap_or(u128::MAX),
            ponder,
        })
    }
}