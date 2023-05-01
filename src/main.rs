use std::env;

use cadabra::*;

fn main() {    
    let args: Vec<String> = env::args().collect();

    println!("{} {} by {}", PKG_NAME, PKG_VERSION, PKG_AUTHORS);

    // Benchmarking
    if args.contains(&"bench".to_string()) {
        run_bench(args.contains(&"save".to_string()));
        return;
    }

    /*let ec = TranspositionTable::new(16).entry_count() as u64;
    let hash = Position::start_pos().zobrist_hash;
    println!("entries : \t{:#065b}", ec - 1);
    println!("hash :    \t{:#065b}", hash);
    println!("index   : \t{:#065b}", hash & ec - 1);
    println!("Modulo  : \t{:#065b}", hash % ec);*/


    // Normal engine run
    // Begin accepting commands
    interface_loop();
}