use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.expect("Failed to read line")).collect()
}

pub fn read_lines_as<T: FromStr>(path: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    read_lines(path)
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

pub fn read_input(day: u8) -> Vec<String> {
    let path = format!("day{:02}/input.txt", day);
    read_lines(&path)
}
