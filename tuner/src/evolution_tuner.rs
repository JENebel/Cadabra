use std::{collections::VecDeque, thread, time::{Duration, Instant}};
use cadabra::{SearchArgs, Settings, WeightArray, CONST_EVALUATOR};
use futures::future::join_all;

use rand::distributions::weighted::alias_method::Weight;
use tokio::task;

use crate::tuner_evaluator::TunerEvaluator;

const POPULATION_SIZE: usize = 4;
const SURVIVORS: usize = 1;
const MUTATION_RATE: f64 = 0.005;
const MUTATION_SIZE: f64 = 1.1;

/// How many games to play with each other individual in each generation
const GAMES: usize = 1;

pub async fn tune() {
    let before = Instant::now();
    
    let population = reproduce(vec![TunerEvaluator::default().get_weights()]);

    run_generation(population).await;

    println!("Done in {:?}", before.elapsed());
}

fn reproduce(survivors: Vec<WeightArray>) -> Vec<WeightArray> {
    // Reproduce up to the population size, and mutate
    let mut new_population = survivors.clone();
    for i in 0..POPULATION_SIZE {
        let parent = survivors[i % survivors.len()].clone();
        let mut child = parent.clone();
        for j in 0..child.len() {
            if fastrand::f64() < MUTATION_RATE {
                let mut new = child[j] as f64 + (1. + fastrand::f64()).powf(2.);
                if fastrand::bool() {
                    new = -new;
                }
                //println!("Mutating weight {} from {} to {}", j, child[j], new);
                child[j] = new as i16;
            }
        }
        new_population.push(child);
    }
    new_population
}

async fn run_generation(population: Vec<WeightArray>) -> Vec<WeightArray> {
    let mut tasks = Vec::new();
    for i in 0..POPULATION_SIZE {
        for j in i+1..POPULATION_SIZE {
            let p1 = population[i].clone();
            let p2 = population[j].clone();
            for _ in 0..GAMES {
                tasks.push(task::spawn(duel(p1, p2)));
                //u += 1;
                tasks.push(task::spawn(duel(p2, p1)));
                println!("{i} v {j}");
                println!("{j} v {i}");
                //u += 1;
            }
        }
    }

    let results = join_all(tasks).await;
    let mut deq: VecDeque<_> = results.into_iter().map(|x| x.unwrap()).collect();
    println!("{:?}", deq);
    let mut scores = vec![0; POPULATION_SIZE];
    for i in 0..POPULATION_SIZE {
        for j in i+1..POPULATION_SIZE {
            for _ in 0..GAMES {
                let result = deq.pop_front().unwrap();
                scores[i] += result as i32;
                scores[j] -= result as i32;
                println!("{i} v {j}: {:?}", result);

                let result = deq.pop_front().unwrap();
                scores[i] -= result as i32;
                scores[j] += result as i32;
                println!("{j} v {i}: {:?}", result);
            }
        }
    }

    // Find the best survivors. 
    // Inefficient copilot solution.
    // TODO
    let mut survivors = Vec::new();
    for _ in 0..SURVIVORS {
        let mut best = 0;
        for j in 1..POPULATION_SIZE {
            if scores[j] > scores[best] {
                best = j;
            }
        }
        survivors.push(population[best].clone());
        scores[best] = std::i32::MIN;
    }

    survivors
}

#[repr(i16)]
#[derive(Debug, Clone, Copy)]
enum GameResult {
    White = 1,
    Black = -1,
    Draw = 0,
}

async fn duel(a: WeightArray, b: WeightArray) -> GameResult {
    let evaluator_a = TunerEvaluator::from_weights(a);
    let evaluator_b = TunerEvaluator::from_weights(b);

    let mut pos = cadabra::Position::start_pos();
    while !pos.generate_moves().len() == 0 {
        let args = SearchArgs::new(None, false, false, None, None, None, None, Some(100)).unwrap();

        let eval = match pos.active_color {
            cadabra::Color::White => evaluator_a,
            cadabra::Color::Black => evaluator_b,
        };
        let search = cadabra::Search::new(Settings::default());
        search.start(pos.clone(), args, false, eval);
    }

    todo!()
}