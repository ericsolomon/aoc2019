use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<i32> = reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(input: &Vec<i32>) {
    println!("part one: {}", input.iter().fold(0, |acc, x| acc + x / 3 - 2));
}

fn part_two(input: &Vec<i32>) {
    println!("part two: {}", input.iter().fold(0, |acc, x| acc + get_fuel(x)));
}

fn get_fuel(input: &i32) -> i32 {
    let fuel = input / 3 - 2;

    match fuel {
        fuel if fuel > 0 => fuel + get_fuel(&fuel),
        _ => 0,
    }
}
