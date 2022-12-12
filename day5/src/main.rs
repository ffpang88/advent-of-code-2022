use std::collections::VecDeque;

use advent_of_code_common;

fn main() {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut is_parsing_stacks = true;

    advent_of_code_common::cli_read_file_by_line(|line| {
        if line.is_empty() {
            is_parsing_stacks = false;
            return;
        }

        if stacks.len() == 0 {
            initialize_stacks(&mut stacks, (0..line.len()).step_by(4).len());
        }

        if is_parsing_stacks {
            parse_for_stacks(&mut stacks, line);
        }
        else {
            operate_crane(&mut stacks, line, crate_mover_9001);
        }
    });

    for stack in stacks {
        print!("{}", stack.get(0).expect("non empty stack"));
    }
}

fn initialize_stacks(stacks: &mut Vec<VecDeque<char>>, num_stacks: usize) {
    (0..num_stacks).for_each(|_| stacks.push(VecDeque::new()));
}

fn parse_for_stacks(stacks: &mut Vec<VecDeque<char>>, line: String) {
        let mut stack_index = 0;
        for i in (0..line.len()).step_by(4) {
            let crate_value = &line[i..i+3].chars().nth(1).expect("crate contains at least 2 characters.");
            if !crate_value.is_whitespace() {
                stacks[stack_index].push_back(crate_value.clone());
            }

            stack_index += 1;
        }
}

fn operate_crane<F>(stacks: &mut Vec<VecDeque<char>>, line: String, mut crate_mover: F) where
    F: FnMut(&mut VecDeque<char>, &mut VecDeque<char>, i32) {
    let mut instruction_iter = line.split(" ");
    instruction_iter.next();
    let num_crates_to_move = instruction_iter.next().expect("at least 2 entries in instruction").parse::<i32>().expect("number of crates to move");
    instruction_iter.next();
    let move_from_stack_index = instruction_iter.next().expect("at least 4 entries in instruction").parse::<usize>().expect("stack number to move from") - 1;
    instruction_iter.next();
    let move_to_stack_index = instruction_iter.next().expect("at least 6 entries in instruction").parse::<usize>().expect("stack number to move to") - 1;

    if move_from_stack_index == move_to_stack_index {
        return;
    }

    let from_stack;
    let to_stack;

    if move_from_stack_index < move_to_stack_index {
        let (front, back) = stacks.as_mut_slice().split_at_mut(move_from_stack_index + 1);
        from_stack = &mut front[move_from_stack_index];
        to_stack = &mut back[move_to_stack_index - move_from_stack_index - 1];
    }
    else {
        let (front, back) = stacks.as_mut_slice().split_at_mut(move_to_stack_index + 1);
        to_stack = &mut front[move_to_stack_index];
        from_stack = &mut back[move_from_stack_index - move_to_stack_index - 1];
    }

    crate_mover(from_stack, to_stack, num_crates_to_move);
}

fn crate_mover_9001(from_stack: &mut VecDeque<char>, to_stack: &mut VecDeque<char>, num_crates_to_move: i32) {
    (0..num_crates_to_move as usize).rev().for_each(|i| to_stack.push_front(from_stack.get(i).expect("element exists").clone()));
    (0..num_crates_to_move).for_each(|_| { from_stack.pop_front(); });
}

fn crate_mover_9000(from_stack: &mut VecDeque<char>, to_stack: &mut VecDeque<char>, num_crates_to_move: i32) {
    (0..num_crates_to_move).for_each(|_| to_stack.push_front(from_stack.pop_front().expect("from stack not empty")));
}
