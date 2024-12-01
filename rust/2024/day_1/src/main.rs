use std::fs::File;
use std::io::prelude::*;

fn difference(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    let mut left_working_list = left_list.clone();
    let mut right_working_list = right_list.clone();
    left_working_list.sort();
    right_working_list.sort();

    while left_working_list.len() > 0 {
        let num1 = left_working_list.pop().unwrap();
        let num2 = right_working_list.pop().unwrap();
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
    total
}

fn similarity(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut total = 0;
    for left_number in left_list {
        for right_number in right_list {
            if left_number == right_number {
                total += *left_number
            }
        }
    }
    total
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let mut numbers = line.split("   ");
        let left_num: i32 = numbers.next().unwrap().parse().unwrap();
        let right_num: i32 = numbers.next().unwrap().parse().unwrap();

        left_list.push(left_num);
        right_list.push(right_num);
    }

    println!("Difference: {}", difference(&left_list, &right_list));
    println!("Similarity Score: {}", similarity(&left_list, &right_list));
    Ok(())
}
