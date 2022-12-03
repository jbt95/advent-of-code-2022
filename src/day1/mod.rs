use std::fs;
use std::path;
use std::path::Path;

pub fn run() {
    let mut results: Vec<usize> = Vec::new();
    let mut batch: Vec<usize> = Vec::new();
    fs::read_to_string(Path::new("./src/day1/input.txt"))
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .enumerate()
        .for_each(|(i, line)| {
            if line.is_empty() {
                results.push(batch.iter().sum::<usize>());
                batch.clear()
            } else {
                batch.push(line.parse::<usize>().unwrap());
            }
        });
    results.sort_by(|a, b| b.cmp(a));
    println!("{:?}", results[0]);
    println!("{:?}", results[0] + results[1] + results[2]);
}
