use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;

mod day_one;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = open_file(args.get(1).unwrap());
    // Change day here as needed (maybe experiment with a switch for which day?)
    let answer = day_one::part_one(lines);
    println!("Answer: {}", answer);
}

fn open_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut list = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        list.push(line);
    }

    return list;
}

