use std::{fs, io::Read};

#[derive(Debug, Copy, Clone)]
struct Dial {
    value: u8,
}
impl Default for Dial {
    fn default() -> Self {
        Self { value: 50 }
    }
}
impl Dial {
    fn underflowing_sub(&mut self, amount: isize) -> usize {
        let mut zeroes = 0;
        if self.value == 0 {
            self.value += 100;
        }
        let mut new = self.value as isize - amount;
        while new < 0 {
            new += 100;
            zeroes += 1;
        }
        self.value = new as u8;
        if self.value == 0 {
            zeroes += 1
        }
        zeroes
    }
    fn overflowing_add(&mut self, amount: usize) -> usize {
        let mut zeroes = 0;
        let mut new = self.value as usize + amount;
        while new > 99 {
            new -= 100;
            zeroes += 1;
        }
        self.value = new as u8;
        zeroes
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct Safe {
    dial: Dial,
    password: usize,
}
impl Safe {
    fn turn_left(&mut self, amount: isize) {
        self.password += self.dial.underflowing_sub(amount);
    }
    fn turn_right(&mut self, amount: usize) {
        self.password += self.dial.overflowing_add(amount);
    }
}

fn main() {
    let mut file = fs::File::open("./input.txt").expect("Error opening file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Error reading file");
    let mut safe = Safe::default();
    for line in input.lines() {
        let (direction, amount) = line.split_at(1);
        let amount: isize = amount.parse().unwrap();
        match direction {
            "L" => safe.turn_left(amount),
            "R" => safe.turn_right(amount as usize),
            _ => (),
        }
    }
    println!("{}", safe.password);
}
