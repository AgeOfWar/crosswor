#![allow(dead_code)]

use std::time::Instant;

use crossword::Crossword;
use matcher::Matcher;

mod crossword;
mod trie;
mod matcher;

fn main() {
    let mut rng = rand::thread_rng();
    let matcher = Matcher::from_file("words.italian.txt").unwrap();
    let mut crossword = Crossword::from_str("
        □ □ □ □ □ ■ □ □ □ □ □ ■
        □ □ □ □ ■ □ □ □ □ □ □ □
        □ □ □ ■ □ □ □ □ □ ■ □ □
        □ □ ■ □ □ □ □ □ ■ □ □ □
        □ ■ □ □ □ □ □ ■ □ □ □ □
        □ □ □ □ □ □ ■ □ □ □ □ □
        □ □ □ □ □ ■ □ □ □ □ □ □
        □ □ □ □ ■ □ □ □ □ □ ■ □
        □ □ □ ■ □ □ □ □ □ ■ □ □
        □ □ ■ □ □ □ □ □ ■ □ □ □
        □ □ □ □ □ □ □ ■ □ □ □ □
        ■ □ □ □ □ □ ■ □ □ □ □ □
    ");
    println!("{}", crossword);

    let start = Instant::now();
    let success = crossword.fill(&matcher, &mut rng);
    let elapsed = start.elapsed();
    println!();
    if success {
        println!("Success!");
    } else {
        println!("Failure!");
    }
    println!();
    println!("{}", crossword);
    println!("{}.{:03} seconds", elapsed.as_secs(), elapsed.subsec_millis());
}
