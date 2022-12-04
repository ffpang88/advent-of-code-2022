use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filepath: String
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.filepath);
    let display = path.display();

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("couldn't open {}: {}", display, reason),
    };

    const HAND_ROCK: char = 'A';
    const HAND_PAPER: char = 'B';
    const HAND_SCISSORS: char = 'C';

    const PLAYER_LOSE: char = 'X';
    const PLAYER_TIE: char = 'Y';
    const PLAYER_WIN: char = 'Z';

    let round_score_map = HashMap::from([
        (PLAYER_WIN, 6),
        (PLAYER_TIE, 3),
        (PLAYER_LOSE, 0),
    ]);

    let hand_score_map = HashMap::from([
        (HAND_ROCK, 1),
        (HAND_PAPER, 2),
        (HAND_SCISSORS, 3),
    ]);

    let round_outcome_map = HashMap::from([
        (format!("{} {}", HAND_ROCK, PLAYER_LOSE).to_owned(), HAND_SCISSORS),
        (format!("{} {}", HAND_ROCK, PLAYER_TIE).to_owned(), HAND_ROCK),
        (format!("{} {}", HAND_ROCK, PLAYER_WIN).to_owned(), HAND_PAPER),
        (format!("{} {}", HAND_PAPER, PLAYER_LOSE).to_owned(), HAND_ROCK),
        (format!("{} {}", HAND_PAPER, PLAYER_TIE).to_owned(), HAND_PAPER),
        (format!("{} {}", HAND_PAPER, PLAYER_WIN).to_owned(), HAND_SCISSORS),
        (format!("{} {}", HAND_SCISSORS, PLAYER_LOSE).to_owned(), HAND_PAPER),
        (format!("{} {}", HAND_SCISSORS, PLAYER_TIE).to_owned(), HAND_SCISSORS),
        (format!("{} {}", HAND_SCISSORS, PLAYER_WIN).to_owned(), HAND_ROCK),
    ]);

    let mut total_score = 0;
    for line in BufReader::new(file).lines() {
        let round = line.unwrap();
        if round.is_empty() {
            continue;
        }
        match round_outcome_map.get(&round) {
            Some(player_hand) => total_score += round_score_map.get(&(round.chars().last().unwrap())).unwrap_or(&0) + hand_score_map.get(&player_hand).unwrap_or(&0),
            None => continue,
        }
    }

    println!("{}", total_score);
}
