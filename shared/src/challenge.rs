use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Instant;

use crate::config::NTHREADS;
use crate::message::{MD5HashCash, MD5HashCashInput, MD5HashCashOutput};

pub trait Challenge {
    /// Données en entrée du challenge
    type Input;
    /// Données en sortie du challenge
    type Output;
    /// Nom du challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// Résout le challenge
    fn solve(&self) -> Self::Output;
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: Self::Output) -> bool;
}

impl Challenge for MD5HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCash(input)
    }

    fn solve(&self) -> Self::Output {
        let now = Instant::now();
        let seed_counter = Arc::new(AtomicU64::new(0));
        let is_solved = Arc::new(AtomicBool::new(false));
        let (worker_tx, worker_rx) = mpsc::channel();
        for _ in 0..NTHREADS {
            let worker_tx = worker_tx.clone();
            let seed_counter = seed_counter.clone();
            let is_solved = is_solved.clone();
            let message = self.0.message.to_string();
            let complexity = self.0.complexity;
            thread::spawn(move || {
                loop {
                    if is_solved.load(Ordering::Relaxed) {
                        break;
                    }
                    let seed = seed_counter.fetch_add(1, Ordering::Relaxed);
                    let hash = md5::compute(format!("{:016X}", seed) + &message);
                    let md5 = format!("{:032X}", hash);
                    if !check_hash(complexity, md5.clone()) {
                        continue;
                    }
                    is_solved.store(true, Ordering::Relaxed);
                    worker_tx.send(MD5HashCashOutput { seed, hashcode: md5.to_string() }).unwrap();
                }
            });
        }
        let elapsed = now.elapsed();
        println!("Thread creation time elapsed 1: {:.2?}", elapsed);
        let out = worker_rx.recv().unwrap();
        let elapsed = now.elapsed();
        println!("Challenge solve time elapsed 2: {:.2?}", elapsed);
        out
    }

    fn verify(&self, _: Self::Output) -> bool {
        todo!()
    }
}

fn check_hash(mut complexity: u32, hash: String) -> bool {
    let bit_compare = 1 << 127;
    let mut sum = u128::from_str_radix(&*hash, 16).unwrap();
    while complexity > 0 {
        if (sum & bit_compare) > 0 {
            break;
        }
        sum = sum << 1;
        complexity -= 1;
    }
    complexity == 0
}
