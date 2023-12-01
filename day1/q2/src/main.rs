use std::fs::File;
use std::io::{BufReader, BufRead};

fn check_match(index: usize, line: String, matching_string: String) -> bool {
    for i in 0..matching_string.len() {
        if index + i >= line.len() {
            return false;
        }
        if line.chars().nth(index + i) != matching_string.chars().nth(i) {
            return false;
        }
    }
    return true;
}

fn match_numbers(index: usize, line: String) -> i32 {
    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for i in 0..numbers.len() {
        if check_match(index, line.clone(), numbers[i].to_string()) {
            return (i + 1) as i32;
        }
    }

    return 0;
}


fn process_line(line: String) -> i32 {
    let mut first = 0;
    let mut last = 0;
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        let number;
        if c > '0' && c <= '9' {
            number = c as i32 - '0' as i32;
        } else {
            number = match_numbers(i, line.clone());
        }
        if number != 0 {
            if first == 0 {
                first = number;
            }
            last = number;
        }
    }
    return first * 10 + last;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {

        sum += process_line(line.unwrap());

        // println!("{}", line.unwrap());
    }
    println!("{}", sum);
}