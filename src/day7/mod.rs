use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread::current;

pub fn run() {
    let file = fs::read_to_string(Path::new("./src/day7/input.txt")).unwrap();
    let mut filesystem: HashMap<PathBuf, HashSet<(i32, &str)>> = HashMap::new();
    let mut pwd = PathBuf::new();
    for line in file.split("$").skip(1) {
        match line.trim().lines().next().unwrap() {
            "ls" => {
                filesystem
                    .entry(pwd.clone())
                    .or_insert(HashSet::new())
                    .extend(
                        line.lines()
                            .skip(1)
                            .map(|output| {
                                output
                                    .split_whitespace()
                                    .collect_tuple()
                                    .map(|(size, file)| (size.parse::<i32>().unwrap_or(-1), file))
                                    .unwrap()
                            })
                            .collect::<Vec<(i32, &str)>>(),
                    );
            }
            "cd .." => {
                pwd.pop();
            }
            cd_path => {
                pwd.push(cd_path.split_once(' ').unwrap().1);
            }
        }
    }
    let mut sizes = HashMap::new();
    for k in filesystem.keys() {
        compute_dir_size(&filesystem, &mut sizes, k);
    }
    let total_size = sizes[&PathBuf::from("/")];
    println!(" {}", sizes.values().filter(|&&s| s <= 100000).sum::<i32>());
    println!(
        "{}",
        sizes
            .values()
            .filter(|&&s| 40000000 + s >= total_size)
            .min()
            .copied()
            .unwrap()
    );
}

fn compute_dir_size(
    fs: &HashMap<PathBuf, HashSet<(i32, &str)>>,
    sizes: &mut HashMap<PathBuf, i32>,
    dir: &PathBuf,
) {
    if sizes.contains_key(dir) {
        return;
    }
    let size = fs[dir]
        .iter()
        .map(|&(s, d)| match s {
            -1 => {
                let dir = dir.join(d);
                compute_dir_size(fs, sizes, &dir);
                sizes[&dir]
            }
            s => s,
        })
        .sum();
    sizes.insert(dir.clone(), size);
}
