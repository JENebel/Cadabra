//mod texel_tuner;
mod pgn_to_fen;
mod tuner_evaluator;
mod evolution_tuner;

#[tokio::main]
async fn main() {
    evolution_tuner::tune().await;
}