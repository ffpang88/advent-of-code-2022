use advent_of_code_common;

use std::collections::HashMap;

fn main() {
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
    advent_of_code_common::cli_read_file_by_line(|round| {
        if round.is_empty() {
            return;
        }
        match round_outcome_map.get(&round) {
            Some(player_hand) => total_score += round_score_map.get(&(round.chars().last().unwrap())).unwrap_or(&0) + hand_score_map.get(&player_hand).unwrap_or(&0),
            None => return,
        }
    });

    println!("{}", total_score);
}
