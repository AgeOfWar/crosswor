#![allow(dead_code)]

use std::{time::{Instant, Duration}, ops::Div};

use crossword::Crossword;
use matcher::Matcher;

mod crossword;
mod trie;
mod matcher;

fn main() {
    let mut rng = rand::thread_rng();
    let matcher = Matcher::from_file("words.italian.txt").unwrap();
    // let mut crossword = Crossword::from_str("
    //     □ □ □ ■ □ □ □ □ ■ □ □ □ □ □ □
    //     □ □ □ ■ □ □ □ □ ■ □ □ □ □ □ □
    //     □ □ □ ■ □ □ □ □ □ □ □ □ □ □ □
    //     □ □ □ □ □ ■ ■ □ □ □ ■ □ □ □ □
    //     □ □ □ □ □ □ □ □ □ □ □ □ ■ ■ ■
    //     ■ ■ ■ □ □ □ □ ■ ■ □ □ □ □ □ □
    //     □ □ □ □ ■ □ □ □ □ ■ □ □ □ □ □
    //     □ □ □ ■ □ □ □ □ □ □ □ ■ □ □ □
    //     □ □ □ □ □ ■ □ □ □ □ ■ □ □ □ □
    //     □ □ □ □ □ □ ■ ■ □ □ □ □ ■ ■ ■
    //     ■ ■ ■ □ □ □ □ □ □ □ □ □ □ □ □
    //     □ □ □ □ ■ □ □ □ ■ ■ □ □ □ □ □
    //     □ □ □ □ □ □ □ □ □ □ □ ■ □ □ □
    //     □ □ □ □ □ □ ■ □ □ □ □ ■ □ □ □
    //     □ □ □ □ □ □ ■ □ □ □ □ ■ □ □ □
    // ");
    let crossword = Crossword::from_str("
        □ □ □ ■ □ □ □ □ ■ □ □ □ □ □ □
        □ □ □ ■ □ □ □ □ ■ □ □ □ □ □ □
        □ □ □ ■ □ □ □ □ □ □ □ □ □ □ □
        □ □ □ □ □ ■ ■ □ □ □ ■ □ □ □ □
        □ □ □ □ □ □ □ □ □ □ □ □ ■ ■ ■
    ");
    
    println!("{}", crossword);

    let mut sum = Duration::ZERO;
    let count = 20u32;

    for i in 0..count {
        let mut crossword = crossword.clone();
        let start = Instant::now();
        let success = crossword.fill(&matcher, &mut rng);
        if !success { panic!("Failed to fill crossword"); }
        let elapsed = start.elapsed();
        sum += elapsed;
        println!("\n{}", crossword);
        println!("{}/{}: {}.{:03} seconds", i+1, count, elapsed.as_secs(), elapsed.subsec_millis());
    }
    
    println!();
    println!("Total: {}.{:03} seconds", sum.as_secs(), sum.subsec_millis());
    println!("Average: {}.{:03} seconds", sum.div(count).as_secs(), sum.div(count).subsec_millis());
}
