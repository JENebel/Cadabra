//use rand::{Rng};

use std::time::SystemTime;

use rand::Rng;

use super::*;

const MAX_PLY: usize = 64;
const FULL_DEPTH_MOVES: u8 = 4;
const REDUCTION_LIMIT: u8 = 3;
pub const MATE_VALUE: i32 = 49000;
pub const MATE_BOUND: i32 = 48000; //Lower bound for mating score
const INFINITY: i32 = 50000;

const INPUT_POLL_INTERVAL: u64 = 16383;

pub fn find_random(pos: &mut Position) {
    let moves = MoveGenerator::initialize(&mut pos, MoveTypes::All).collect();
    let rand = rand::thread_rng().gen_range(0..moves.len());
    print!("bestmove {}\n", moves[rand].to_uci());
}

pub struct SearchResult {
    pub best_move: Move,
    pub nodes_visited: u64,
    pub score: i32,
    pub depth: u8,
    pub reached_max_ply: bool,
    pub tt_hits: u32
}

impl SearchResult {
    pub fn new(cmove: Move, nodes: u64, score: i32, depth: u8, reached_max_ply: bool, tt_hits: u32) -> Self {
        Self { best_move: cmove, nodes_visited: nodes, score: score, depth: depth, reached_max_ply: reached_max_ply, tt_hits: tt_hits }
    }
}

//Start a search, max_time = -1 for no limit
pub fn search(game: &mut Position, depth: i8, max_time: i64, io_receiver: &IoWrapper, tt: &mut TranspositionTable, rep_table: &mut RepetitionTable) -> SearchResult {

    let mut envir = SearchEnv::new(max_time, io_receiver, tt, rep_table);

    let mut score = 0;

    let mut alpha = -INFINITY;
    let mut beta  =  INFINITY;

    let mut current_depth: u8 = 1;
    let max_depth = if depth == -1 { MAX_PLY as u8 } else { depth as u8 };

    while current_depth <= max_depth as u8 {
        envir.follow_pv = true;

        score = negamax(game, current_depth as u8, alpha, beta, &mut envir);

        if envir.stopping { break }

        //Narrowing aspiration window
        if score <= alpha || score >= beta {
            alpha = -INFINITY;
            beta  =  INFINITY;

            current_depth += 1;

            continue;
        }

        alpha = score - 50;
        beta  = score + 50;

        if score >= -MATE_VALUE && score < -MATE_BOUND {
            print!("info score mate {} depth {} nodes {} time {} pv ", -(score + MATE_VALUE) / 2 - 1, current_depth, envir.nodes, envir.start_time.elapsed().unwrap().as_millis());
        }
        else if score <= MATE_VALUE && score > MATE_BOUND {
            print!("info score mate {} depth {} nodes {} time {} pv ", (MATE_VALUE - score) / 2 + 1, current_depth, envir.nodes, envir.start_time.elapsed().unwrap().as_millis());
        }
        else {
            print!("info score cp {} depth {} nodes {} time {} pv ", score, current_depth, envir.nodes, envir.start_time.elapsed().unwrap().as_millis());
        }

        for i in 0..envir.pv_lengths[0] {
            print!("{} ", envir.pv_table[0][i].to_uci());
        }
        print!("\n");
        
        current_depth += 1;
    }

    print!("bestmove {}\n", envir.pv_table[0][0].to_uci());

    SearchResult::new(envir.pv_table[0][0], envir.nodes, score, current_depth - 1, !envir.stopping, envir.tt_hits)
}

#[inline]
fn negamax(pos: &mut Position, depth: u8, alpha: i32, beta: i32, envir: &mut SearchEnv) -> i32 {
    
    let is_pv_node = (beta - alpha) > 1;

    let mut score;
    if envir.ply != 0 && !is_pv_node {
        score = envir.transposition_table.probe(pos.zobrist_hash, depth, alpha, beta, envir.ply);
        if score != UNKNOWN_SCORE {
            envir.tt_hits += 1;
            return score;
        }
    }

    envir.pv_lengths[envir.ply as usize] = envir.ply as usize;

    if envir.ply > 0 && envir.repetition_table.is_now_in_threefold_repetition() {
        return 0;
    }

    //Dont't go on if reached max ply
    if envir.ply >= MAX_PLY as u8 - 1  {
        return evaluate(&pos);
    }

    if envir.nodes & INPUT_POLL_INTERVAL == 0 {
        envir.poll_input()
    }

    if depth == 0 || pos.half_moves == 100 {
        //return evaluate(game)
        return quiescence(pos, alpha, beta, envir);
    }

    let mut hash_flag = HashFlag::Alpha;

    envir.nodes += 1;

    let in_check = pos.is_in_check(pos.active_player);

    let n_depth = if in_check { depth + 1 } else { depth };

    let mut temp_alpha = alpha;

    let mut legal_moves = 0;

    //Null move pruning
    if n_depth >= 3 && !in_check && envir.ply > 0 {
        let mut copy = *pos;

        //Switch side + update hash
        copy.active_player = opposite_color(copy.active_player);
        copy.zobrist_hash ^= SIDE_KEY;

        //Reset enpassant + update hash
        if copy.enpassant_square != Square::None {
            copy.zobrist_hash ^= ENPASSANT_KEYS[copy.enpassant_square as usize];
        };
        copy.enpassant_square = Square::None;

        //..., Depth - 1 - R (with R = 2), ...

        envir.ply += 1;

        score = -negamax(&mut copy, n_depth - 1 - 2, -beta, -beta + 1, envir);

        envir.ply -= 1;

        if envir.stopping { return 0 }

        //Cut-off
        if score >= beta {
            return beta
        }
    }

    let mut moves = MoveGenerator::initialize(pos, MoveTypes::All);

    let mut moves_searched = 0;

    while moves.has_next() {
        let m = moves.get_next_move(true);
        
        let mut copy = pos.clone();

        envir.ply += 1;

        if !copy.make_move(&m, &mut envir.repetition_table) { 
            envir.ply -= 1; 
            
            continue;
        }

        legal_moves += 1;

        envir.repetition_table.move_back();

        if moves_searched == 0 {
            //Full PV Search
            score = -negamax(&mut copy, n_depth - 1, -beta, -temp_alpha, envir);
        } else {
            //Regular search with LMR

            score = if  moves_searched >= FULL_DEPTH_MOVES && 
                        depth >= REDUCTION_LIMIT &&
                        !in_check &&
                        !m.is_capture() &&
                        m.promotion() == Piece::None as u8 {
                //Reduced search
                -negamax(&mut copy, n_depth - 2, -temp_alpha - 1, -temp_alpha, envir)

            } else {
                //Ensure a full search
                temp_alpha + 1
            };

            //PVS
            if score > temp_alpha {
                score = -negamax(&mut copy, n_depth - 1, -temp_alpha - 1, -temp_alpha, envir);

                //Check bounds
                if score > temp_alpha && score < beta {
                    //Full search on failure
                    score = -negamax(&mut copy, n_depth - 1, -beta, -temp_alpha, envir);
                }
            }
        }

        envir.ply -= 1;

        moves_searched += 1;

        if envir.stopping { return 0 }

        if score > temp_alpha {
            //Insert PV node
            envir.insert_pv_node(m);

            //Beta cut-off
            if score >= beta {
                //Update killer moves
                if !m.is_capture() {
                    envir.killer_moves[1][envir.ply as usize] = envir.killer_moves[0][envir.ply as usize];
                    envir.killer_moves[0][envir.ply as usize] = Some(m);
                }
    
                //Record TT entry
                envir.transposition_table.record(pos.zobrist_hash, beta, depth, HashFlag::Beta, envir.ply);
    
                return beta;
            }

            //Record TT entry
            hash_flag = HashFlag::Exact;

            //Update history move
            if !m.is_capture() {
                envir.history_moves[m.piece() as usize][m.to_square() as usize] += depth as i32
            }

            temp_alpha = score;
        }
    }

    //Mate & Draw
    if legal_moves == 0 {
        if in_check {
            return -MATE_VALUE + envir.ply as i32;
        }
        else {
            return 0;
        }
    }
    
    //Record TT entry
    envir.transposition_table.record(pos.zobrist_hash, temp_alpha, depth, hash_flag, envir.ply);

    temp_alpha
}

#[inline]
fn quiescence(pos: &mut Position, alpha: i32, beta: i32, envir: &mut SearchEnv) -> i32 {
    if envir.nodes & INPUT_POLL_INTERVAL == 0 {
        envir.poll_input()
    }

    envir.nodes += 1;

    let eval = evaluate(&pos);

    //Dont't go on if reached max ply
    if envir.ply > MAX_PLY as u8 - 1 || pos.half_moves == 100 {
        return eval;
    }

    let mut temp_alpha = alpha;

    if eval > temp_alpha {
        temp_alpha = eval;

        if eval >= beta {
            return beta
        }
    }

    let mut moves = MoveGenerator::initialize(pos, MoveTypes::Captures);

    while moves.has_next() {
        let m = moves.get_next_move(true);

        let mut copy = *pos;
        if !copy.make_move(&m, &mut envir.repetition_table) {
            continue;
        }
        
        envir.ply += 1;

        let score = -quiescence(&mut copy, -beta, -temp_alpha, envir);

        envir.ply -= 1;

        envir.repetition_table.move_back();

        if score >= beta {
            return beta;
        }

        if score > temp_alpha {
            temp_alpha = score;
        }
    }

    temp_alpha
}

pub struct SearchEnv<'a> {
    pub nodes: u64,
    pub ply: u8,
    pub killer_moves: [[Option<Move>; MAX_PLY]; 2],
    pub history_moves: [[i32; 64]; 12],
    pub pv_lengths: [usize; MAX_PLY],
    pub pv_table: [[Move; MAX_PLY]; MAX_PLY],
    pub follow_pv: bool,
    pub score_pv: bool,
    pub stopping: bool,
    io_receiver: &'a IoWrapper,
    pub start_time: SystemTime,
    max_time: i64,
    transposition_table: &'a mut TranspositionTable,
    pub tt_hits: u32,
    pub repetition_table: &'a mut RepetitionTable,
}

impl <'a>SearchEnv<'a> {
    pub fn new(max_time: i64, io_receiver: &'a IoWrapper, tt: &'a mut TranspositionTable, rep_table: &'a mut RepetitionTable) -> Self {
        Self{
            nodes: 0,
            ply: 0,
            killer_moves: [[None; MAX_PLY]; 2],
            history_moves: [[0 as i32; 64]; 12],
            pv_lengths: [0; MAX_PLY],
            pv_table: [[NULL_MOVE; MAX_PLY]; MAX_PLY],
            follow_pv: false,
            score_pv: false,
            stopping: false,
            io_receiver: io_receiver,
            start_time: SystemTime::now(),
            max_time: max_time,
            transposition_table: tt,
            tt_hits: 0,
            repetition_table: rep_table
        }
    }

    pub fn insert_pv_node(&mut self, cmove: Move) {
        let ply = self.ply as usize;

        self.pv_table[ply][ply] = cmove;
        
        for next_ply in (ply + 1)..self.pv_lengths[ply + 1] {
            self.pv_table[ply][next_ply] = self.pv_table[ply + 1][next_ply];
        }

        self.pv_lengths[ply] = self.pv_lengths[ply + 1];
    }

    pub fn poll_input(&mut self) {
        if (self.max_time != -1 && self.start_time.elapsed().unwrap().as_millis() as i64 >= self.max_time) || self.io_receiver.try_read_line().is_some() {
            self.stopping = true;
            return;
        }
    }
}