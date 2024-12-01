use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    for line in contents.lines() {
        let mut numbers = line.split("   ");
        let left_num: i32 = numbers.next().unwrap().parse().unwrap();
        let right_num: i32 = numbers.next().unwrap().parse().unwrap();

        left_list.push(left_num);
        right_list.push(right_num);
    }

    left_list.sort();
    right_list.sort();

    while left_list.len() > 0 {
        let num1 = left_list.pop().unwrap();
        let num2 = right_list.pop().unwrap();
        let larger;
        let smaller;
        if num1 > num2 {
            larger = num1;
            smaller = num2;
        } else {
            larger = num2;
            smaller = num1;
        }
        total += larger - smaller;
    }

    println!("{}", total);
    Ok(())
}
