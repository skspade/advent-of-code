use std::env;

mod days;
mod utils;

use utils::read_input;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        eprintln!("Example: {} 1", args[0]);
        std::process::exit(1);
    }

    let day: u8 = args[1].parse().expect("Day must be a number 1-12");

    if day < 1 || day > 12 {
        eprintln!("Day must be between 1 and 12");
        std::process::exit(1);
    }

    let input = read_input(day);

    println!("Day {:02}", day);
    println!("========");

    let (part1, part2) = run_day(day, &input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn run_day(day: u8, input: &[String]) -> (String, String) {
    match day {
        1 => (days::day01::part1(input), days::day01::part2(input)),
        2 => (days::day02::part1(input), days::day02::part2(input)),
        3 => (days::day03::part1(input), days::day03::part2(input)),
        4 => (days::day04::part1(input), days::day04::part2(input)),
        5 => (days::day05::part1(input), days::day05::part2(input)),
        6 => (days::day06::part1(input), days::day06::part2(input)),
        7 => (days::day07::part1(input), days::day07::part2(input)),
        8 => (days::day08::part1(input), days::day08::part2(input)),
        9 => (days::day09::part1(input), days::day09::part2(input)),
        10 => (days::day10::part1(input), days::day10::part2(input)),
        11 => (days::day11::part1(input), days::day11::part2(input)),
        12 => (days::day12::part1(input), days::day12::part2(input)),
        _ => unreachable!(),
    }
}
