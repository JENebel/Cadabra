mod tuner;
mod pgn_to_fen;
mod tuner_evaluator;

use tuner::*;
use pgn_to_fen::*;

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No file provided!");
        std::process::exit(1);
    });

    if input.ends_with("pgn") {
        // Generate fen file
        let pgn_path = std::path::PathBuf::from(input.trim());
        let fen_path = generate_fen_from_pgn(pgn_path);
        tune(fen_path)
    } else if input.ends_with("fen") {
        let fen_path = std::path::PathBuf::from(input.trim());
        tune(fen_path)
    } else {
        println!("Invalid file type");
    }
}