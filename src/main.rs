extern crate core;

use std::io::{BufRead, Read};
use regex::Regex;

const VERSION_1: i32 = 9000;
const VERSION_1_1: i32 = 9001;

struct MoveCommand {
    crates_count: usize,
    src_stack: usize,
    dest_stack: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let first_opt = args.get(1);
    let crate_mover_version;
    if first_opt.is_none() {
        crate_mover_version = VERSION_1;
    } else {
        crate_mover_version = match first_opt.unwrap().parse().unwrap() {
            VERSION_1 => VERSION_1,
            VERSION_1_1 => VERSION_1_1,
            _ => panic!("Unknown version"),
        }
    }

    let crate_lines = get_crate_lines();
    let mut stacks: Vec<Vec<char>> = get_crate_stacks(&crate_lines);
    apply_move_commands(&mut stacks, crate_mover_version);
    print_top_crates(&stacks);
}

fn print_top_crates(stacks: &Vec<Vec<char>>) {
    print!("Crates on top of the stacks: ");
    for stack in stacks {
        if stack.is_empty() {
            println!();
        } else {
            print!("{}", stack.last().unwrap());
        }
    }
    println!();
}

fn apply_move_commands(mut stacks: &mut Vec<Vec<char>>, crate_mover_version: i32) {
    println!("Input move commands:\n");
    let re = regex::Regex::new(r"^move ([\d]{1,}) from ([\d]{1,}) to ([\d]{1,})").unwrap();
    for line_res in std::io::stdin().lines() {
        let line = line_res.unwrap();
        if line.is_empty() {
            break;
        }

        let cmd_res = build_move_command(&re, line);
        if cmd_res.is_none() {
            break;
        }

        apply_move(&mut stacks, cmd_res.unwrap(), crate_mover_version);
    }
}

fn build_move_command(re: &Regex, line: String) -> Option<MoveCommand> {
    let cap_res = re.captures(line.as_str());
    if cap_res.is_none() {
        return None;
    }
    let cap_opt = cap_res.unwrap();

    let cmd = MoveCommand {
        crates_count: cap_opt.get(1).unwrap().as_str().to_string().parse().unwrap(),
        src_stack: cap_opt.get(2).unwrap().as_str().to_string().parse().unwrap(),
        dest_stack: cap_opt.get(3).unwrap().as_str().to_string().parse().unwrap(),
    };

    Some(cmd)
}

fn apply_move(stacks: &mut Vec<Vec<char>>, command: MoveCommand, crate_mover_version: i32) {
    if crate_mover_version == VERSION_1_1 {
        let mut tmp_stack: Vec<char> = vec![];
        for i in 0..command.crates_count {
            let opt = stacks[command.src_stack - 1].pop();
            tmp_stack.push(opt.unwrap());
        }
        tmp_stack.reverse();
        tmp_stack.retain(|item| {
            stacks[command.dest_stack - 1].push(*item);
            false
        });
    } else {
        for i in 0..command.crates_count {
            let opt = stacks[command.src_stack - 1].pop();
            stacks[command.dest_stack - 1].push(opt.unwrap());
        }
    }
}

fn get_crate_stacks(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut stacks_num: usize = 0;

    for i in (0..lines.len()).rev() {
        let chars = parse_line(&lines[i]);

        if stacks_num == 0 {
            stacks_num = chars.len();
        }

        let mut y: usize = 0;
        for each_char in chars {

            if each_char != ' ' {
                if stacks.get(y).is_none() {
                    let mut stack: Vec<char> = vec![];
                    stacks.push(stack);
                }

                stacks[y].push(each_char);
            }

            y += 1;
        }
    }

    stacks
}

fn get_crate_lines() -> Vec<String> {
    println!("Input stacks with crates:\n");

    let mut lines: Vec<String> = vec![];
    for line in std::io::stdin().lines() {
        let chars = line.unwrap();
        if chars.is_empty() {
            break;
        }
        lines.push(chars);
    }

    lines
}

fn parse_line(line: &String) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    let mut i: usize = 0;
    for each_char in line.chars() {
        if i > 0 && (i - 1) % 4 == 0 {
            result.push(each_char);
        }
        i += 1;
    }
    result
}
