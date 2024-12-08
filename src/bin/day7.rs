use std::{collections::HashMap, fs};

fn recursive_find(target: i128, num_vec: Vec<i128>, depth: i128) -> bool {
    // Base cases:
    // Length of vec = 1 and vec = target, then good
    if target < 0 {
        return false;
    }
    if num_vec.len() == 1 {
        return num_vec[0] == target;
    }

    // Otherwise, check addition branch first
    let (lhs, rhs) = num_vec.split_at(2);

    let mut add_rhs = rhs.to_vec();
    let mut mult_rhs = rhs.to_vec();

    // Prep multiplication branch
    add_rhs.insert(0, lhs.iter().sum());
    mult_rhs.insert(0, lhs.iter().product());

    return recursive_find(target, add_rhs, depth + 1)
        || recursive_find(target, mult_rhs, depth + 1);
}

fn part_7b_recursive(target: i128, num_vec: Vec<i128>) -> bool {
    if num_vec.len() == 1 {
        return num_vec[0] == target;
    }

    let (lhs, rhs) = num_vec.split_at(2);

    let mut add_rhs = rhs.to_vec();
    let mut mult_rhs = rhs.to_vec();
    let mut concat_rhs = rhs.to_vec();

    // Prep multiplication branch
    add_rhs.insert(0, lhs.iter().sum());
    mult_rhs.insert(0, lhs.iter().product());

    concat_rhs.insert(0, lhs[0] * 10_i128.pow(lhs[1].ilog10() + 1 as u32) + lhs[1]);

    return part_7b_recursive(target, add_rhs)
        || part_7b_recursive(target, mult_rhs)
        || part_7b_recursive(target, concat_rhs);
}

fn main() {
    let fileread = fs::read_to_string("inputs/day7.txt").expect("Unable to read file as string");

    let mut test_map: HashMap<i128, Vec<i128>> = HashMap::new();

    for line in fileread.lines() {
        let (lhs, rhs) = line.split_once(":").unwrap();
        let lhs_int = lhs.parse::<i128>().unwrap();
        let rhs_vec = rhs
            .split_whitespace()
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        test_map.insert(lhs_int, rhs_vec);
    }

    let mut running_sum = 0;
    let mut part_7b_sum = 0;

    for (k, v) in test_map.iter() {
        let found_result = recursive_find(*k, v.clone(), 0);

        if found_result {
            running_sum += k;
        }

        let found_part_b = part_7b_recursive(*k, v.clone());

        if found_part_b {
            part_7b_sum += k;
        }
    }
    // Final result gt 698_802_628_957
    // Final result is 945_512_582_195
    println!("Final running sum: {}", running_sum);
    println!("Part B running sum: {}", part_7b_sum);
}
