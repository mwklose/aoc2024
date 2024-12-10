use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn add_antinodes<'a>(hs: &mut HashSet<(usize, usize)>, entry_vec: Vec<&(usize, usize)>) {
    let first = entry_vec.first().unwrap();
    let last = entry_vec.last().unwrap();

    // Method: 2*anchor - other
    // For anchor = first
    if 2 * first.0 >= last.0 && 2 * first.1 >= last.1 {
        let first_antinode = (2 * first.0 - last.0, 2 * first.1 - last.1);
        hs.insert(first_antinode);
    }

    // For anchor = last
    if 2 * last.0 >= first.0 && 2 * last.1 >= first.1 {
        let last_antinode = (2 * last.0 - first.0, 2 * last.1 - first.1);
        hs.insert(last_antinode);
    }
}

fn add_multiantinodes(
    hs: &mut HashSet<(usize, usize)>,
    entry_vec: Vec<&(usize, usize)>,
    max_rows: usize,
    max_cols: usize,
) {
    let first = entry_vec.first().unwrap();
    let last = entry_vec.last().unwrap();

    let mut mult = 1;

    while mult * first.0 >= (mult - 1) * last.0
        && mult * first.1 >= (mult - 1) * last.1
        && (mult * first.0 - (mult - 1) * last.0) < max_rows
        && (mult * first.1 - (mult - 1) * last.1) < max_cols
    {
        let first_antinode = (
            mult * first.0 - (mult - 1) * last.0,
            mult * first.1 - (mult - 1) * last.1,
        );
        hs.insert(first_antinode);
        mult += 1;
    }

    let mut mult = 1;

    while mult * last.0 >= (mult - 1) * first.0
        && mult * last.1 >= (mult - 1) * first.1
        && (mult * last.0 - (mult - 1) * first.0) < max_rows
        && (mult * last.1 - (mult - 1) * first.1) < max_cols
    {
        let last_antinode = (
            mult * last.0 - (mult - 1) * first.0,
            mult * last.1 - (mult - 1) * first.1,
        );
        hs.insert(last_antinode);
        mult += 1;
    }
}

fn main() {
    let fileread = fs::read_to_string("inputs/day8.txt").expect("Unable to read file as string");
    let mut antennas: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

    fileread.lines().enumerate().for_each(|(row, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c.ne(&'.'))
            .for_each(|(col, c)| {
                let hs = antennas
                    .entry(c)
                    .or_insert(HashSet::<(usize, usize)>::new());
                hs.insert((row, col));
            })
    });

    let nrows = fileread.lines().count();
    let ncols = fileread.lines().next().unwrap().chars().count();

    // Iterate through pairs in hashset to find antinodes.
    let mut running_count_hs = HashSet::new();
    let mut part_8b_hs = HashSet::new();

    antennas.iter().for_each(|(k, v)| {
        println!("Starting {}, with {} items", k, v.len());
        v.into_iter().combinations(2).for_each(|vec| {
            add_antinodes(&mut running_count_hs, vec.clone());
            add_multiantinodes(&mut part_8b_hs, vec.clone(), nrows, ncols);
        });
    });

    println!(
        "Antinodes: {}",
        running_count_hs
            .iter()
            .filter(|(n1, n2)| { *n1 < nrows && *n2 < ncols })
            .count()
    );

    println!("Multiple Antinodes: {}", part_8b_hs.iter().count());
}
