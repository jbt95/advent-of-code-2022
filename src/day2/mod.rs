use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn run() {
    let scores: HashMap<String, usize> = HashMap::from([
        (String::from("AZ"), 3),
        (String::from("AX"), 4),
        (String::from("AY"), 8),
        (String::from("BX"), 1),
        (String::from("BY"), 5),
        (String::from("BZ"), 9),
        (String::from("CX"), 7),
        (String::from("CY"), 2),
        (String::from("CZ"), 6),
        (String::from("AA"), 4),
        (String::from("BB"), 5),
        (String::from("CC"), 6),
    ]);

    let win: HashMap<String, String> = HashMap::from([
        (String::from("A"), String::from("Y")),
        (String::from("B"), String::from("Z")),
        (String::from("C"), String::from("X")),
    ]);

    let lose: HashMap<String, String> = HashMap::from([
        (String::from("A"), String::from("Z")),
        (String::from("B"), String::from("X")),
        (String::from("C"), String::from("Y")),
    ]);
    println!(
        "{:?}",
        fs::read_to_string(Path::new("./src/day2/input.txt"))
            .unwrap()
            .lines()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|line| {
                let players = line.split_whitespace().collect::<Vec<&str>>();

                return (players[0], players[1]);
            })
            .map(|(p1, p2)| match p2 {
                "Z" => scores
                    .get(&format!("{}{}", p1, win.get(p1).unwrap()))
                    .unwrap(),
                "X" => scores
                    .get(&format!("{}{}", p1, lose.get(p1).unwrap()))
                    .unwrap(),
                "Y" => scores.get(&format!("{}{}", p1, p1)).unwrap(),
                _ => panic!("Invalid move"),
            })
            .sum::<usize>()
    );
}
