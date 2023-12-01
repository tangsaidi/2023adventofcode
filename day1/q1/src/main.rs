use std::fs::File;
use std::io::{BufReader, BufRead};

fn process_line(line: String) -> i32 {
    let mut first = '0';
    let mut last = '0';
    for c in line.chars() {
        if c > '0' && c <= '9' {
            if first == '0' {
                first = c;
            }
            last = c;

        }
    }
    return (first as i32 -'0' as i32) * 10 + (last as i32 - '0' as i32);
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