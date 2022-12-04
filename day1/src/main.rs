use advent_of_code_common;

use std::fmt::Debug;

fn main() {
    const N: usize = 3;

    let mut top_n_calories: [i32; N] = [0; N];
    let mut cur_calories = 0;
    advent_of_code_common::cli_read_file_by_line(|calories_string| {
        if !calories_string.is_empty() {
            cur_calories += calories_string.parse::<i32>().unwrap();
            return;
        }

        bubble_up(&mut top_n_calories, cur_calories.clone());

        cur_calories = 0;
    });

    print!("{}", top_n_calories.into_iter().reduce(|a, b| a + b).unwrap());
}

fn bubble_up<T: PartialOrd + Debug, const N: usize>(cur_top_ranking: &mut [T; N], to_add: T) {
    let last_index = cur_top_ranking.len() - 1;
    if to_add <= cur_top_ranking[last_index] {
        return;
    }

    cur_top_ranking[last_index] = to_add;

    for i in (0 .. last_index).rev() {
        if cur_top_ranking[i + 1] <= cur_top_ranking[i] {
            break;
        }

        cur_top_ranking.swap(i, i + 1);
    }
}
