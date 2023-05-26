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

    // Normal engine run
    // Begin accepting commands
    interface_loop();
}