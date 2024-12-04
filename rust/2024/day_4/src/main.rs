use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Puzzle {
    length: usize,
    height: usize,
    grid: HashMap<(usize, usize), char>,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut puzzle_string: String = String::from("");
        for x in 0..self.length {
            for y in 0..self.height {
                let character = self.grid.get(&(x, y)).unwrap().to_owned();
                puzzle_string.push(character);
                if y == self.length - 1 {
                    puzzle_string.push('\n');
                }
            }
        }

        write!(
            f,
            "length: {}
height: {}
---grid---
{}",
            self.length, self.height, puzzle_string
        )
    }
}

fn create_puzzle(input: String) -> Puzzle {
    let grid = {
        let mut list = HashMap::new();
        for line in input.lines().enumerate() {
            for char in line.1.chars().enumerate() {
                list.insert((char.0, line.0), char.1);
            }
        }
        list
    };

    Puzzle {
        length: input.lines().next().unwrap().len(),
        height: input.lines().count(),
        grid,
    }
}

fn parse(line: &str) -> usize {
    let revline: String = line.chars().rev().collect::<String>();
    let mut total = line.matches("XMAS").count();
    total += revline.matches("XMAS").count();

    total
}

fn make_vertical_list(puzzle: &Puzzle) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for x in 0..=(puzzle.length - 1) {
        let mut line: String = String::new();
        for y in 0..=(puzzle.height - 1) {
            line.push(puzzle.grid.get(&(x, y)).unwrap().to_owned());
        }
        output.push(line);
    }
    output
}

fn make_horizontal_list(puzzle: &Puzzle) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for y in 0..=(puzzle.height - 1) {
        let mut line: String = String::new();
        for x in 0..=(puzzle.length - 1) {
            line.push(puzzle.grid.get(&(x, y)).unwrap().to_owned())
        }
        output.push(line);
    }

    output
}

fn make_diagonal_list(puzzle: &Puzzle) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for _ in 0..=(puzzle.height - 1) {
        let mut line: String = String::new();
        for x in 0..=(puzzle.length - 1) {
            let mut i = x + 1;
            while i < puzzle.length {
                line.push(puzzle.grid.get(&(x, i)).unwrap().to_owned());
                i += 1;
            }
        }
        output.push(line);
    }

    for _ in puzzle.height..=0 {
        let mut line: String = String::new();
        for x in puzzle.length..=0 {
            let mut i = x - 1;
            while i > 0 {
                line.push(puzzle.grid.get(&(x, i)).unwrap().to_owned());
                i -= 1;
            }
        }
        output.push(line);
    }

    println!("{:?}", output);
    output
}

fn find_matches(puzzle: Puzzle) -> usize {
    let horizontals = make_horizontal_list(&puzzle);
    let verticals = make_vertical_list(&puzzle);
    let diagonals = make_diagonal_list(&puzzle);

    let mut horizontal_total = 0;
    let mut verticals_total = 0;
    let mut diagonals_total = 0;
    for line in horizontals {
        horizontal_total += parse(&line);
    }
    for line in verticals {
        verticals_total += parse(&line);
    }
    for line in diagonals {
        diagonals_total += parse(&line);
    }
    println!(
        "horizontals: {}\nverticals: {}\ndiagonals: {}",
        horizontal_total, verticals_total, diagonals_total
    );

    horizontal_total + verticals_total + diagonals_total
}

fn main() {
    let mut file = File::open("example").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let puzzle = create_puzzle(contents);

    let mut count = 0;
    count += find_matches(puzzle);

    println!("Total matches: {count}");
}
