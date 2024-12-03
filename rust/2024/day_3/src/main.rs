use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

static EXAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
static EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[derive(Debug)]
enum State {
    Do,
    Dont,
    Command,
}

struct Command {
    index: usize,
    state: State,
    contents: Option<String>,
}

fn detect_commands(text: &str) -> Vec<String> {
    let exp = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don\'t\(\)").unwrap();
    let mut decision_list: Vec<Command> = Vec::new();

    for do_match in do_regex.find_iter(text) {
        decision_list.push(Command {
            index: do_match.start(),
            state: State::Do,
            contents: None,
        });
    }

    for dont_match in dont_regex.find_iter(text) {
        decision_list.push(Command {
            index: dont_match.start(),
            state: State::Dont,
            contents: None,
        });
    }

    for matched_phrase in exp.find_iter(text) {
        decision_list.push(Command {
            index: matched_phrase.start(),
            state: State::Command,
            contents: Some(matched_phrase.as_str().to_owned()),
        })
    }

    decision_list.sort_by_key(|cmd| cmd.index);

    let mut process_command = true;
    let mut command_list: Vec<String> = Vec::new();
    for command in decision_list {
        if !process_command {
            match command.state {
                State::Do => process_command = true,
                _ => (),
            }
        } else {
            match command.state {
                State::Command => {
                    for matched_phrase in exp.find_iter(text) {
                        if command.index == matched_phrase.start() {
                            command_list.push(command.contents.clone().unwrap());
                        }
                    }
                }
                State::Dont => process_command = false,
                State::Do => (),
            }
        }
    }

    command_list
}

fn multiply(text: &str) -> usize {
    let string = text.replace("mul(", "").replace(")", "");
    let left_number: usize = string.split(",").nth(0).unwrap().parse().unwrap();
    let right_number: usize = string.split(",").nth(1).unwrap().parse().unwrap();

    left_number * right_number
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut total = 0;
    //for command in detect_commands(EXAMPLE2) {
    //    total += multiply(&command);
    //}
    for line in contents.lines() {
        for command in detect_commands(line) {
            total += multiply(&command);
        }
    }
    println!("{total}");
}
