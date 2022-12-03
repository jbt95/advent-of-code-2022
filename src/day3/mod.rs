use std::collections::hash_set::Intersection;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn run() {
    println!(
        "{}",
        fs::read_to_string(Path::new("./src/day3/input.txt"))
            .unwrap()
            .lines()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(split_arr)
            .map(|(left, right)| -> (HashSet<char>, HashSet<char>) {
                (HashSet::from_iter(left), HashSet::from_iter(right))
            })
            .map(|(left, right)| {
                left.intersection(&right)
                    .cloned()
                    .collect::<HashSet<char>>()
                    .into_iter()
                    .next()
                    .unwrap()
            })
            .map(|char| {
                if char.is_lowercase() {
                    return char as i32 - 96;
                }
                return char as i32 - 38;
            })
            .sum::<i32>()
    );
}

fn split_arr(line: &str) -> (Vec<char>, Vec<char>) {
    let chars = line.chars().collect::<Vec<char>>();
    let len = chars.len();

    return (chars[..len / 2].to_vec(), chars[len / 2..].to_vec());
}
