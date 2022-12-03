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
            .chunks(3)
            .map(|(chunk)| {
                (
                    chunk[0].chars().collect::<Vec<char>>(),
                    chunk[1].chars().collect::<Vec<char>>(),
                    chunk[2].chars().collect::<Vec<char>>(),
                )
            })
            .map(|(a, b, c)| {
                return intersection(intersection(a, b).into_iter().collect::<Vec<char>>(), c)
                    .into_iter()
                    .next()
                    .unwrap();
            })
            .map(|char| {
                if char.is_lowercase() {
                    return char as usize - 96;
                }
                return char as usize - 38;
            })
            .sum::<usize>()
    );
}

fn intersection(left: Vec<char>, right: Vec<char>) -> HashSet<char> {
    let a: HashSet<char> = HashSet::from_iter(left);
    let b: HashSet<char> = HashSet::from_iter(right);

    return a.intersection(&b).cloned().collect::<HashSet<char>>();
}
