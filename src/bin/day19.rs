use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use itertools::Itertools;

fn count_towel_matches(towel_stack: &Vec<&str>, target_towel: &str) -> i64 {
    // Let's make non-recursive version
    let mut running_targets: Vec<&str> = Vec::new();
    let mut count_map: HashMap<&str, i64> = HashMap::new();
    count_map.insert(target_towel, 1);
    running_targets.push(target_towel);

    while let Some(tt) = running_targets.pop() {
        let substrings = towel_stack
            .iter()
            .filter(|&&t| tt.starts_with(t))
            .map(|&t| tt.strip_prefix(t).unwrap())
            .collect_vec();

        for substring in substrings {
            if count_map.contains_key(substring) {
                match count_map.get(substring) {
                    Some(x) => {
                        count_map.insert(substring, x + count_map.get(tt).unwrap());
                    }
                    _ => panic!("Impossible to not get match"),
                }
            } else {
                let prior_count = *count_map.get(tt).unwrap();

                count_map.insert(substring, prior_count);
                running_targets.push(substring);
            }
        }

        running_targets.sort_by_key(|&k| k.len());
    }

    let running_count = count_map.get("").unwrap();

    return *running_count;
}

fn match_towels(towel_stack: &Vec<&str>, target_towel: &str) -> bool {
    // Let's make non-recursive version
    let mut running_targets: VecDeque<&str> = VecDeque::new();
    let mut already_processed: HashSet<&str> = HashSet::new();
    running_targets.push_back(target_towel);

    while let Some(tt) = running_targets.pop_front() {
        already_processed.insert(tt);
        let substrings = towel_stack
            .iter()
            .filter(|&&t| tt.starts_with(t))
            .map(|&t| tt.strip_prefix(t).unwrap())
            .collect_vec();

        for substring in substrings {
            if substring.len() == 0 {
                return true;
            }

            if !already_processed.contains(&substring) {
                running_targets.push_back(substring);
            }
        }
    }

    return false;
}

fn main() {
    let fileread = fs::read_to_string("inputs/day19.txt").expect("Unable to read file as string");

    let mut lines_iter = fileread.lines();

    let towels = match lines_iter.next() {
        Some(x) => x.split(", ").collect_vec(),
        _ => panic!("Should be able to read possible towels"),
    };

    // Throw away empty line
    lines_iter.next();

    let mut hs: HashSet<&str> = HashSet::new();
    let mut unique_vec: Vec<i64> = Vec::new();

    while let Some(line) = lines_iter.next() {
        println!("Working on line: \n\t{}", line);
        let res = match_towels(&towels, line);
        if res {
            println!("\t\tAble to match!\n");
            hs.insert(line);
            let num_matches = count_towel_matches(&towels, line);

            println!("\t\tFound {} unique matches", num_matches);
            unique_vec.push(num_matches);
        } else {
            println!("\t\tNo match.");
        }
    }

    println!("Part A: {} towels possible", hs.len());

    println!(
        "Part B: {} unique towel combos",
        unique_vec.iter().fold(0, |acc, x| acc + x)
    );
}
