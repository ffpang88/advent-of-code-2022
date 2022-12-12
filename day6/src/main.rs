use std::{collections::{VecDeque, HashSet}, str::CharIndices};

use advent_of_code_common;

fn main() {
    advent_of_code_common::cli_read_file_by_line(|line| {
        const WINDOW_SIZE: usize = 14;
        let mut window = VecDeque::new();
        let mut char_indices = line.char_indices();
        while let Some((offset, char)) = char_indices.next() {
            while window.len() > 0 && window.contains(&char) {
                window.pop_front();
            }
            if window.len() < WINDOW_SIZE - 1 {
                window.push_back(char);
            }
            else {
                println!("{}", offset + 1);
                break;
            }
        }
    });
}
