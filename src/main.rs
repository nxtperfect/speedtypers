use std::io;
use std::time::{Duration, Instant};
use std::thread;

fn main() {
    let starting_text = "Type out the text below as quickly as possible! GO!";
    let speed_text = "the quick brown fox jumped over the lazy dog";
    let mut buffer = String::new();
    println!("{}", starting_text);
    println!("{}", speed_text);
    let start = Instant::now();
    // maybe change it to something like that
    // loop {
    //     // Read input (if any)
    //     let input = stdin.next();
    //
    //     // If a key was pressed
    //     if let Some(Ok(key)) = input {
    //         match key {
    //             // Exit if 'q' is pressed
    //             termion::event::Key::Char('q') => break,
    //             // Else print the pressed key
    //             _ => {
    //                 s.push(key);
    //                 stdout.lock().flush().unwrap();
    //             }
    //         }
    //     }
    //     thread::sleep(time::Duration::from_millis(50));
    // }
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    let duration = start.elapsed();
    if buffer.trim() == speed_text {
        println!("\x1b[1;32mYup that's correct!\x1b[1;39m");
        let wpm = (9.0 / (duration.as_secs() as f64 / 60.0));
        let accuracy = 100;
        println!("You got {}wpm with {}% accuracy and finished typing in {:?}seconds!", wpm, accuracy, duration);
    }
}
