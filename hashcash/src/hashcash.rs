use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::thread;

use crate::config::NTHREADS;
use crate::dto::MD5HashCashOutput;
use crate::utils::check_hash;

pub struct Hashcash;

impl Hashcash {
    pub fn solve(message: String, complexity: u32) -> MD5HashCashOutput {
        let seed_counter = Arc::new(AtomicU64::new(0));
        let is_solved = Arc::new(AtomicBool::new(false));
        let (worker_tx, worker_rx) = mpsc::channel();
        for _ in 0..NTHREADS {
            let worker_tx = worker_tx.clone();
            let seed_counter = seed_counter.clone();
            let is_solved = is_solved.clone();
            let message = message.clone();
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
        worker_rx.recv().unwrap()
    }
}