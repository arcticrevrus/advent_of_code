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
        for y in 0..self.length {
            for x in 0..self.height {
                let character = self.grid.get(&(x, y)).unwrap().to_owned();
                puzzle_string.push(character);
                if x == self.length - 1 {
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
    for x in 0..puzzle.length {
        let mut line: String = String::new();
        for y in 0..puzzle.height {
            line.push(puzzle.grid.get(&(x, y)).unwrap().to_owned());
        }
        output.push(line);
    }
    output
}

fn make_horizontal_list(puzzle: &Puzzle) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for y in 0..puzzle.height {
        let mut line: String = String::new();
        for x in 0..puzzle.length {
            line.push(puzzle.grid.get(&(x, y)).unwrap().to_owned())
        }
        output.push(line);
    }

    output
}

// - | 0 1 2 3 4 5 6 7 8 9
// -----------------------
// 0 | . . . . X X M A S .
// 1 | . S A M X M S . . .
// 2 | . . . S . . A . . .
// 3 | . . A . A . M S . X
// 4 | X M A S A M X . M M
// 5 | X . . . . . X A . A
// 6 | S . S . S . S . S S
// 7 | . A . A . A . A . A
// 8 | . . M . M . M . M M
// 9 | . X . X . X M A S X

fn make_diagonal_list(puzzle: &Puzzle) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    let mut diagonals: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut current_diagonal: Vec<(usize, usize)> = Vec::new();
    for column_start in 0..puzzle.length {
        for i in 0..(puzzle.length - column_start) {
            current_diagonal.push((i, i + column_start));
        }
        diagonals.push(current_diagonal);
        current_diagonal = Vec::new();
    }

    for row_start in 1..puzzle.height {
        for i in 0..(puzzle.height - row_start) {
            current_diagonal.push((i, i + row_start));
        }
        diagonals.push(current_diagonal);
        current_diagonal = Vec::new();
    }

    let mut left_diagonals: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut current_left_diagonal: Vec<(usize, usize)> = Vec::new();
    for column_start in (0..puzzle.length).rev() {
        for i in (0..(puzzle.length - column_start)).rev() {
            current_left_diagonal.push((i, i + column_start));
        }
        left_diagonals.push(current_left_diagonal);
        current_left_diagonal = Vec::new();
    }

    for row_start in (1..puzzle.height).rev() {
        for i in (0..(puzzle.height - row_start)).rev() {
            current_left_diagonal.push((i, i + row_start));
        }
        left_diagonals.push(current_left_diagonal);
        current_left_diagonal = Vec::new();
    }

    let mut line: String = String::new();
    for diagonal in diagonals {
        for coordinate in diagonal {
            line.push(puzzle.grid.get(&coordinate).unwrap().to_owned());
        }
        output.push(line);
        line = String::new();
    }
    for diagonal in left_diagonals {
        for coordinate in diagonal {
            line.push(puzzle.grid.get(&coordinate).unwrap().to_owned());
        }
        output.push(line);
        line = String::new();
    }

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
    let mut file = File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let puzzle = create_puzzle(contents);
    println!("{puzzle}");
    let mut count = 0;
    count += find_matches(puzzle);

    println!("Total matches: {count}");
}
