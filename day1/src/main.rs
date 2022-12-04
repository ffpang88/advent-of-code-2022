use clap::Parser;
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

    const N: usize = 3;

    let mut top_n_calories: [i32; N] = [0; N];
    let mut cur_calories = 0;
    for line in BufReader::new(file).lines() {
        let calories_string = line.unwrap();
        if !calories_string.is_empty() {
            cur_calories += calories_string.parse::<i32>().unwrap();
            continue;
        }

        bubble_up(&mut top_n_calories, cur_calories.clone());

        cur_calories = 0;
    }
    

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
