use std::{io::{BufRead, BufReader}, fs::File};
use std::collections::HashMap;
use std::cmp;

#[derive(Debug)]
struct RevealPair {
    color: String,
    count: i32
}

impl RevealPair {
    fn new(input: &str) -> Self {
        let input: Vec<&str> = input.split_whitespace().collect();
        let count = input[0].parse::<i32>().unwrap();
        let color = input[1].trim().to_string();
        return Self {
            color,
            count
        }
    }

    fn is_possible(&self, config: &HashMap<&str, i32>) -> bool {
        if !config.contains_key(self.color.as_str()) {
            return false;
        }
        let count = config.get(self.color.as_str()).unwrap();
        return count >= &self.count;
    }
}
#[derive(Debug)]
struct Reveal {
    pairs: Vec<RevealPair>
}

impl Reveal {
    fn new(input: &str) -> Self {
        let mut pairs = Vec::new();
        let input: Vec<&str> = input.split(',').collect();
        for pair in input {
            pairs.push(RevealPair::new(pair));
        }
        return Self { pairs };
    }
    
    fn is_possible(&self, config: &HashMap<&str, i32>) -> bool {
        for pair in &self.pairs {
            if !pair.is_possible(&config) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
struct Game {
    reveals: Vec<Reveal>,
}

impl Game {
    fn new(inputs: Vec<&str>) -> Self {
        let mut reveals = Vec::new();

        for input in inputs {
            let reveal = Reveal::new(input);
            reveals.push(reveal);
        }

        Self { reveals }
    }

    fn is_possible(&self, config: &HashMap<&str, i32>) -> bool {
        for reveal in &self.reveals {
            if !reveal.is_possible(config) {
                return false;
            }
        }
        return true;
    }

    fn get_power(&self) -> i32 {
        let mut red = 0;
        let mut green= 0;
        let mut blue = 0;

        for reveal in &self.reveals {
            for pair in &reveal.pairs {
                if pair.color == "red" {
                    red = cmp::max(pair.count, red);
                } else if pair.color == "green" {
                    green = cmp::max(pair.count, green);
                } else if pair.color == "blue" {
                    blue = cmp::max(pair.count, blue);
                }
            }
        }
        return red * green * blue;
    }
}

fn main() {
    let mut possible_counts = 0;
    let mut total_power = 0;

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let config = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    for line in reader.lines() {
        let line = line.unwrap();
        let (game_number , line) = line.split_at(line.find(':').unwrap() + 1);

        let line: Vec<&str> = line.split(';').collect();

        let game = Game::new(line);
        
        if game.is_possible(&config) {
            let game_number: Vec<&str> = game_number.split_whitespace().collect();
            let game_number = game_number[1];
            let game_number = game_number.trim_end_matches(':');
            let game_number = game_number.parse::<i32>().unwrap();

            possible_counts += game_number;
        }
        total_power += game.get_power();
    }
    println!("Possible count: {}", possible_counts);
    println!("Total power: {}", total_power)
}
