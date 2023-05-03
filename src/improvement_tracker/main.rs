use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"save".to_string()) {
        save_bin();
    } else {
        duel();
    }
}

fn save_bin() {

}

fn duel() {

}