use core::panic;
use std::collections::HashMap;

use advent_of_code_common;

fn main() {
    let mut dir_size = HashMap::new();
    let mut current_directory: String = String::new();
    advent_of_code_common::cli_read_file_by_line(|line| {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let num_parts = parts.len();

        if num_parts < 2 || num_parts > 3{
            panic!();
        }

        if parts[1] == "ls" {
            return;
        }

        if parts[1] == "cd" {
            if parts[2] == ".." {
                current_directory.truncate(current_directory.rfind("/").expect("non root dir"));
            }
            else {
                current_directory.push_str("/");
                current_directory.push_str(parts[2]);
            }
        }

        if parts[0] == "$" || parts[0] == "dir"{
            return;
        }

        let file_size = parts[0].parse::<i64>().expect("file size as integer");
        let mut dir = current_directory.clone();
        while !dir.is_empty() {
            dir_size.entry(dir.clone()).and_modify(|cur_size| *cur_size += file_size).or_insert(file_size);
            dir.truncate(dir.rfind("/").expect("at least root"));
        }
    });

    let target = 30000000 - (70000000 - dir_size.get("/").expect("root dir exists"));

    let mut last_smallest = -1;
    for (_, value) in dir_size {
        if value < target {
            continue;
        }
        if last_smallest < 0 || last_smallest > value {
            last_smallest = value;
        }
    }

    println!("{}", last_smallest);

}