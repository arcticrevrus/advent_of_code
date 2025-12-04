use std::{io::Read, ops::IndexMut};

const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[derive(Default, Debug, Clone)]
struct Row {
    positions: Vec<bool>,
}

#[derive(Default, Debug, Clone)]
struct Grid {
    rows: Vec<Row>,
    height: usize,
    width: usize,
}
impl Grid {
    fn new(input: &str) -> Self {
        let height;
        let width;
        Grid {
            rows: {
                let mut rows = Vec::new();
                for line in input.lines().map(|l| l.trim()) {
                    let mut row = Vec::new();
                    for c in line.chars() {
                        match c.to_string().as_str() {
                            "." => row.push(false),
                            "@" => row.push(true),
                            _ => panic!("Invalid input"),
                        }
                    }
                    rows.push(Row { positions: row });
                }
                width = rows[0].positions.len();
                height = rows.len();
                rows
            },
            height,
            width,
        }
    }
    fn get(self, x: isize, y: isize) -> bool {
        if x.is_negative() || y.is_negative() {
            return false;
        }
        let row = match self.rows.get(y as usize) {
            Some(r) => r,
            None => return false,
        };
        *row.positions.get(x as usize).unwrap_or(&false)
    }
    fn get_mut(&mut self, x: isize, y: isize) -> &mut bool {
        self.rows
            .index_mut(y as usize)
            .positions
            .index_mut(x as usize)
    }

    fn remove(&mut self, x: isize, y: isize) {
        *self.get_mut(x, y) = false
    }
    fn check_forklift(&self, x: usize, y: usize) -> bool {
        if !self.clone().get(x as isize, y as isize) {
            return false;
        }
        let checks = vec![
            (x as isize - 1, y as isize - 1),
            (x as isize - 1, y as isize),
            (x as isize - 1, y as isize + 1),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
            (x as isize + 1, y as isize - 1),
            (x as isize + 1, y as isize),
            (x as isize + 1, y as isize + 1),
        ];
        let mut total = 0;
        for check in checks {
            if self.clone().get(check.0, check.1) {
                total += 1;
            }
        }
        total < 4
    }
}

fn main() {
    let mut file = std::fs::File::open("./input").expect("Error opening file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Error reading input file");
    let mut grid = Grid::new(&input);
    let mut check = 0;
    let mut total = 0;
    loop {
        for y in 0..grid.clone().height {
            for x in 0..grid.clone().width {
                if grid.clone().check_forklift(x, y) {
                    total += 1;
                    check += 1;
                    grid.remove(x as isize, y as isize);
                }
            }
        }
        println!("{check}");
        if check == 0 {
            break;
        }
        check = 0;
    }
    println!("{total}")
}
