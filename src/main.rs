use std::io;
use std::time::{Duration, Instant};
use std::thread;

fn main() {
    let starting_text = "Type out the text below as quickly as possible! GO!";
    let speed_text = "the quick brown fox jumped over the lazy dog";
    let mut buffer = String::new();
    println!("{}", starting_text);
    println!("Are you ready? Press enter.");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    println!("{}", speed_text);
    let start = Instant::now();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    let duration = start.elapsed();
    let wpm = (9.0 / (duration.as_secs() as f64 / 60.0));
    let accuracy = 100;
    println!("You got {}wpm with {}% accuracy and finished typing in {:?}!", wpm, accuracy, duration);
}
