use std::env;

use cadabra::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"bench".to_string()) {
        run_bench(args.contains(&"save".to_string()));
        return;
    }

    // Normal engine run
    println!("{} {} by {}", PKG_NAME, PKG_VERSION, PKG_AUTHORS);

    //println!("Actual: {}, Ideal: 8388608", TranspositionTable::new(128).entry_count());

    // Begin accepting commands
    interface_loop();
}