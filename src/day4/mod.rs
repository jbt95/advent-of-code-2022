use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn run() {
    println!(
        "{}",
        fs::read_to_string(Path::new("./src/day4/input.txt"))
            .unwrap()
            .lines()
            .into_iter()
            .map(|line| line.split(",").collect::<Vec<&str>>())
            .map(parse_pairs)
            .map(|(a, b)| -> (HashSet<i32>, HashSet<i32>) {
                (
                    HashSet::from_iter((a[0]..=a[1]).collect::<Vec<i32>>()),
                    HashSet::from_iter((b[0]..=b[1]).collect::<Vec<i32>>()),
                )
            })
            .map(|(a, b)| a.intersection(&b).count() > 0)
            .map(|res| i32::from(res))
            .sum::<i32>()
    );
}

fn parse_pairs(pairs: Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let parse = |pair: &str| {
        pair.split("-")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    };

    return (parse(pairs[0]), parse(pairs[1]));
}
