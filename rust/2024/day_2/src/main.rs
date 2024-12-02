use std::fs::File;
use std::io::prelude::*;

fn build_report(text: &str) -> Vec<i32> {
    let numbers = text.split(" ");
    let mut report: Vec<i32> = Vec::new();
    for number in numbers {
        let value: i32 = number.parse().unwrap();
        report.push(value);
    }
    report
}

fn is_increasing(report: &[i32]) -> bool {
    let mut is_increasing: bool = true;
    let mut previous_value: i32 = report[0];
    for &value in report.iter().skip(1) {
        if value < previous_value {
            is_increasing = false;
            return is_increasing;
        }
        previous_value = value;
    }
    is_increasing
}

fn is_decreasing(report: &[i32]) -> bool {
    let mut is_decreasing: bool = true;
    let mut previous_value: i32 = report[0];
    for &value in report.iter().skip(1) {
        if value > previous_value {
            is_decreasing = false;
            return is_decreasing;
        }
        previous_value = value;
    }
    is_decreasing
}

fn difference(report: &[i32]) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut difference;
    for (index, value) in report.iter().enumerate() {
        if index < report.len() - 1 {
            difference = value - report[index + 1];
            output.push(difference);
        }
    }
    output
}

fn is_safe(report: &[i32]) -> bool {
    let mut is_safe: bool = true;
    let is_increasing = is_increasing(report);
    let is_decreasing = is_decreasing(report);
    if !is_increasing && !is_decreasing {
        is_safe = false;
    }
    for mut value in difference(report) {
        value = value.abs();
        if value < 1 {
            is_safe = false;
        }
        if value > 3 {
            is_safe = false;
        }
    }
    is_safe
}

fn tolerance(report: &Vec<i32>) -> bool {
    let mut safe = false;
    for (index, _value) in report.iter().enumerate() {
        if !safe {
            let mut temp_report = report.clone();
            temp_report.remove(index);
            safe = is_safe(&temp_report);
        }
    }
    safe
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut safety_list: Vec<bool> = Vec::new();
    for line in contents.lines() {
        let report = build_report(line);
        let issafe = tolerance(&report);
        println!("Report: {report:?}, safe: {issafe}");
        safety_list.push(issafe);
    }

    let mut total = 0;

    for i in safety_list {
        if i {
            total += 1;
        }
    }

    println!("{total}");

    Ok(())
}
