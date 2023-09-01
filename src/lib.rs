pub mod args;

use args::Args;
use sha256::digest;
use std::ops::AddAssign;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::available_parallelism;

/// size of numbers to be processed by thread
const BATCH_SIZE: u64 = 1000;

fn mine(tx: &Sender<String>, current_number: u64, suffix: &String) {
    let hash = digest(current_number.to_string());
    if hash.ends_with(suffix) {
        tx.send(format!("{}, \"{}\"", current_number, hash))
            .unwrap();
    }
}

fn run_thread(tx: Sender<String>, counter: Arc<Mutex<u64>>, num_zeros: usize) {
    let suffix = "0".repeat(num_zeros);
    loop {
        let mut num = counter.lock().unwrap();
        let from = *num;
        *num += BATCH_SIZE;
        drop(num);
        for i in from..from + BATCH_SIZE {
            mine(&tx, i, &suffix);
        }
    }
}

pub fn run(args: Args) {
    let counter: Arc<Mutex<u64>> = Arc::new(Mutex::new(1));
    let (tx, rx) = mpsc::channel();
    let num_threads = available_parallelism().unwrap().get();

    for _ in 0..num_threads {
        let tx_cloned = tx.clone();
        let counter = Arc::clone(&counter);
        thread::spawn(move || run_thread(tx_cloned, counter, args.num_zeros));
    }
    let mut entries: usize = 0;
    for received in rx {
        println!("{}", received);
        entries.add_assign(1);
        if entries == args.num_digits_to_find {
            return;
        }
    }
}