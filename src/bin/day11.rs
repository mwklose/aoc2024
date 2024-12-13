use std::{collections::HashMap, fs};

use itertools::Itertools;

fn process_stones(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
        let stone_string = stone.to_string();
        let stone_len = stone_string.len();
        let (lhs, rhs) = stone_string.split_at(stone_len / 2);

        return vec![lhs.parse::<u64>().unwrap(), rhs.parse::<u64>().unwrap()];
    }

    return vec![stone * 2024];
}

struct StoneHolder {
    stone: u64,
    count: u64,
}

fn merge_duplicates(sh_vec: Vec<StoneHolder>) -> Vec<StoneHolder> {
    let mut hm: HashMap<u64, u64> = HashMap::new();

    for sh in sh_vec {
        let maybe_v = hm.get(&sh.stone);

        match maybe_v {
            Some(x) => {
                let newcount = x + sh.count;
                hm.insert(sh.stone, newcount);
            }
            _ => {
                hm.insert(sh.stone, sh.count);
            }
        }
    }

    let hm_vec: Vec<StoneHolder> = hm
        .iter()
        .map(|(k, v)| StoneHolder {
            stone: *k,
            count: *v,
        })
        .collect();

    return hm_vec;
}

fn split_stones(sh_vec: Vec<StoneHolder>) -> Vec<StoneHolder> {
    let mut out_hm: HashMap<u64, u64> = HashMap::new();

    for sh in sh_vec {
        let new_stones = process_stones(sh.stone);
        for stone in new_stones {
            let maybe_v = out_hm.get(&stone);

            match maybe_v {
                Some(x) => {
                    let newcount = x + sh.count;
                    out_hm.insert(stone, newcount);
                }
                _ => {
                    out_hm.insert(stone, sh.count);
                }
            }
        }
    }

    let hm_vec: Vec<StoneHolder> = out_hm
        .iter()
        .map(|(k, v)| StoneHolder {
            stone: *k,
            count: *v,
        })
        .collect();

    return hm_vec;
}

fn main() {
    let fileread = fs::read_to_string("inputs/day11.txt").expect("Unable to read file as string");

    let mut stones = fileread
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    let num_steps = 25;

    (0..num_steps).for_each(|_| {
        stones = stones
            .iter()
            .map(|s| process_stones(*s))
            .flatten()
            .collect()
    });

    // assert_eq!(stones, &[1, 2024, 1, 0, 9, 9, 2021976]); // On 0 1 10 99 999
    //
    println!("{:?}", stones);
    println!("Num stones: {}", stones.len());

    // Part B froze computer, split into smaller parts?
    // Order does not matter, can merge together with count variables instead
    //

    let mut sh_vec: Vec<StoneHolder> = fileread
        .split_whitespace()
        .map(|x| StoneHolder {
            stone: x.parse::<u64>().unwrap(),
            count: 1,
        })
        .collect();

    for i in 0..75 {
        // Merge duplicates
        sh_vec = merge_duplicates(sh_vec);
        // Perform split
        sh_vec = split_stones(sh_vec);
        // Count?

        println!(
            "Iter {} -> {}",
            i,
            sh_vec.iter().fold(0, |acc, x| acc + x.count)
        );
    }
}
