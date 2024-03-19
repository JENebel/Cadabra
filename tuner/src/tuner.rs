use std::{io::{BufReader, BufRead, Write}, fs::{File, OpenOptions}, path::PathBuf, time::Instant/*, time::Instant*/};
use chrono::Local;
use rayon::prelude::*;
use term_size::*;
use num_format::*;

use cadabra::Position;

use crate::tuner_evaluator::TunerEvaluator;

pub fn tune(fen_file: PathBuf) {
    println!("Tuning engine with '{}'", fen_file.display());
    // Time how long the calculation takes
    println!("Loading positions...");
    let start = Instant::now();
    let positions = load_positions(fen_file);
    let duration = start.elapsed();
    print!("\x1B[A");
    println!("\rLoaded {} positions in: {duration:?}", positions.len().to_formatted_string(&Locale::en));

    println!("Calculating best k...");
    let now = Local::now().naive_local();
    let k = find_k(&positions);
    print!("\x1B[A");
    println!("\rFound besk k: {}", k);

    let folder_name = "tuning_results";
    let file_name = format!("tuning_results_{}", now.format("%Y-%m-%d %H.%M.%S").to_string());
    let _ = std::fs::create_dir(folder_name);
    File::create(format!("{folder_name}/{file_name}.txt")).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{folder_name}/{file_name}.txt"))
        .unwrap();

    let before = Instant::now();

    let mut weights = TunerEvaluator::default().get_weights();
    let mut delta = weights.into_iter().map(|_| 1).collect::<Vec<_>>();

    writeln!(file, "Initial weights: {:?}", weights).unwrap();

    //println!("Initial weights: {:?}", weights);
    let mut best_err = mean_square_error(&positions, TunerEvaluator::default(), k);
    println!("Initial error: {}\n", best_err);

    let mut iterations = 0;

    let mut improved = true;
    while improved {
        iterations += 1;
        improved = false;
        let mut adjusted_weights = 0;
        let in_before = Instant::now();

        for w in 0..weights.len() {
            let mut new_weights = weights.clone();

            let step = delta[w];
            new_weights[w] += step;
            let new_err = mean_square_error(&positions, TunerEvaluator::from_weights(new_weights), k);

            if new_err < best_err {
                best_err = new_err;
                weights = new_weights;
                improved = true;
                adjusted_weights += 1;
            } else {
                new_weights[w] -= 2 * step;
                let new_error = mean_square_error(&positions, TunerEvaluator::from_weights(new_weights), k);
                if new_error < best_err {
                    best_err = new_error;
                    weights = new_weights;
                    improved = true;
                    adjusted_weights += 1;
                    delta[w] = -delta[w];
                }
            }

            let s = format!("Iteration {}:   Weight {w}/{}   Time: {}", iterations, weights.len(), pretty_duration::pretty_duration(&in_before.elapsed(), None));
            let spaces = " ".repeat(dimensions_stdout().unwrap().0 - s.len());
            print!("\x1B[A");
            println!("\r{}{}", s, spaces);
            std::io::stdout().flush().unwrap();
        }

        let s = format!("Iteration {}:   Adjusted {adjusted_weights}/{} weights.   Time: {}   Total time elapsed: {}.   Error: {}", iterations, weights.len(), pretty_duration::pretty_duration(&in_before.elapsed(), None), pretty_duration::pretty_duration(&before.elapsed(), None), best_err);
        let spaces = " ".repeat(dimensions_stdout().unwrap().0 - s.len());
        print!("\x1B[A");
        println!("\r{}{}\n", s, spaces);

        // Save weights to file
        writeln!(file, "Iteration {iterations}: {:?}", weights).unwrap();
    }

    let msg = format!("Tuning finished in: {}. Error: {}", pretty_duration::pretty_duration(&before.elapsed(), None), best_err);
    println!("{msg}");
    writeln!(file, "{msg}").unwrap();

    writeln!(file, "\nFinal weights:\n{}", TunerEvaluator::from_weights(weights)).unwrap();
}

pub fn load_positions(fen_file: PathBuf) -> Vec<(f64, Position)> {
    let lines = BufReader::new(File::open(fen_file.clone()).unwrap()).lines().collect::<Vec<_>>();
    let positions: Vec<(f64, Position)>  = lines.par_iter().map(|line| {
        let line = line.as_ref().unwrap();
        let (result, fen) = line.split_once("]").unwrap();
        let outcome = match result.chars().last().unwrap() {
            '1' => 1.,
            '0' => 0.,
            '½' => 0.5,
            _ => panic!("Invalid result"),
        };
        let pos = Position::from_fen(&fen).unwrap();
        (outcome, pos)
    }).collect();

    positions
}

fn mean_square_error(positions: &Vec<(f64, Position)>, evaluator: TunerEvaluator, k: f64) -> f64 {
    let n = positions.len() as usize;
    let error: f64 = positions.par_iter().map(|(outcome, pos)| {
        let mut score = pos.evaluate(evaluator) as f64;
        if pos.active_color == cadabra::Color::Black { score *= -1.; } // Correct score to be independent of color
        (outcome - sigmoid(score, k)).powi(2)
    }).sum();

    error / n as f64
}

/// Finds the k that minimizes the mean square error
fn find_k(positions: &Vec<(f64, Position)>) -> f64 {
    let evaluator = TunerEvaluator::default();
    let mut best_k = 1f64;
    let mut best_err = mean_square_error(positions, evaluator, best_k);
    let mut step = 1f64;
    for _ in 0..10 {
        let mut new_k = best_k + step as f64;
        let new_err = mean_square_error(positions, evaluator, new_k);
        if new_err < best_err {
            best_err = new_err;
            best_k = new_k;
        } else {
            new_k = best_k - step as f64;
            let new_err = mean_square_error(positions, evaluator, new_k);
            if new_err < best_err {
                best_err = new_err;
                best_k = new_k;
                step = -step;
            } else {
                step /= 2.;
            }
        }
    }
    best_k
}

fn sigmoid(s: f64, k: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf(-k * s / 400.))
}