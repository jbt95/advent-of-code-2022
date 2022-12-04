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
            .map(
                |(a, b)| match a[0] <= b[0] && a[1] >= b[1] || b[0] <= a[0] && b[1] >= a[1] {
                    true => 1,
                    false => 0,
                },
            )
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
