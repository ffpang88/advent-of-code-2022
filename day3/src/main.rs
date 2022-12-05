use std::collections::HashSet;

use advent_of_code_common;

fn main() {

    let mut item_set: HashSet<char> = HashSet::new();
    let mut total_priority = 0;
    let mut counter = 0;
    advent_of_code_common::cli_read_file_by_line(|rucksack| {
        counter += 1;

        let mut current_set = HashSet::new();

        rucksack.chars().for_each(|c| { current_set.insert(c); });

        if item_set.is_empty() {
            item_set = current_set;
            return;
        }

        let mut intersection = HashSet::new();
        item_set.intersection(&current_set).for_each(|c| { intersection.insert(c.clone()); });

        item_set = intersection;
        
        if counter < 3 {
            return;
        }

        for c in &item_set {
            total_priority += to_priority(c.clone());
        }

        counter = 0;
        item_set.clear();
    });

    println!("{}", total_priority);
}

fn to_priority(char: char) -> u32 {
    const LOWER_A:u32 = 'a' as u32;
    const LOWER_Z:u32 = 'z' as u32;
    const UPPER_A:u32 = 'A' as u32;
    const UPPER_Z:u32 = 'Z' as u32;

    let char_value = char as u32;

    let offset = if char_value >= LOWER_A && char_value <= LOWER_Z {
        char_value - LOWER_A
    }
    else if char_value >= UPPER_A && char_value <= UPPER_Z {
        char_value - UPPER_A + 26
    }
    else {
        panic!("unexpected character");
    };

    offset + 1
}
