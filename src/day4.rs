use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread::spawn;
use std::path::Path;
use std::fs;
use std::env;

extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

const OK_SIGNAL: &str = "00000";
const OK_SIGNAL_PT2: &str = "000000";

fn mine_adventcoins(key: &str, signal: &str) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let found = Arc::new(AtomicBool::new(false));

    let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();

    for _ in 0..8 {
        let thread_found = found.clone();
        let thread_counter = counter.clone();
        let thread_tx = tx.clone();
        let thread_key = key.to_string().clone();
        let thread_signal = signal.to_string().clone();
        let mut thread_md5 = Md5::new();

        spawn(move || loop {
            if thread_found.load(Ordering::Relaxed) {
                break;
            }

            let n = thread_counter.fetch_add(1, Ordering::SeqCst);

            thread_md5.reset();
            thread_md5.input_str(&format!("{}{}", thread_key, n));

            if thread_md5.result_str().starts_with(&thread_signal) {
                thread_tx.send(n).unwrap();
                thread_found.store(true, Ordering::Relaxed);
                break;
            }
        });
    }

    rx.recv().unwrap()
}

#[aoc(day4, part1)]
pub fn run(input: &str) -> String {
    let res = mine_adventcoins(input, OK_SIGNAL).to_string();

    save_answer(&res, "day4.1");

    res
}

#[aoc(day4, part2)]
pub fn run_pt2(input: &str) -> String {
    let res = mine_adventcoins(input, OK_SIGNAL_PT2).to_string();

    save_answer(&res, "day4.2");

    res
}

pub fn save_answer(ans: &str, part: &str){
    let ans_path = get_current_working_dir();
    let ans_path = Path::new(&ans_path).parent().unwrap().parent().unwrap().parent().unwrap().join("ans");
    let file_path = ans_path.join(format!("{}.txt", part));
    fs::write(file_path, ans).expect("Unable to write file");
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

#[test]
fn test_run() {
    assert_eq!(mine_adventcoins("abcdef", OK_SIGNAL), 609_043);
    assert_eq!(mine_adventcoins("pqrstuv", OK_SIGNAL), 1_048_970);
}
