use std::{collections::HashSet, fs};

use itertools::Itertools;

fn trail_unwrapper(
    hs: &mut HashSet<(usize, usize)>,
    val: i32,
    coord: Option<HashSet<(usize, usize)>>,
) {
    if val == 0 {
        return;
    }

    if coord.is_none() {
        return;
    }

    let hs_to_insert = coord.unwrap();

    for item in hs_to_insert {
        hs.insert(item);
    }
}
fn find_trails(
    bitmap: &Vec<Vec<i32>>,
    search_num: i32,
    rowpos: usize,
    colpos: usize,
) -> (i32, Option<HashSet<(usize, usize)>>) {
    let max_rows = bitmap.len();
    let max_cols = bitmap[0].len();

    if rowpos >= max_rows {
        return (0, None);
    }

    if colpos >= max_cols {
        return (0, None);
    }

    if bitmap[rowpos][colpos] == search_num {
        if search_num == 9 {
            return (1, Some(HashSet::from([(rowpos, colpos)])));
        }

        let (upval, upcoord) = match rowpos > 0 {
            true => find_trails(bitmap, search_num + 1, rowpos - 1, colpos),
            _ => (0, None),
        };

        let (downval, downcoord) = match (rowpos + 1) >= max_rows {
            true => (0, None),
            _ => find_trails(bitmap, search_num + 1, rowpos + 1, colpos),
        };

        let (leftval, leftcoord) = match colpos > 0 {
            true => find_trails(bitmap, search_num + 1, rowpos, colpos - 1),
            _ => (0, None),
        };

        let (rightval, rightcoord) = match (colpos + 1) >= max_cols {
            true => (0, None),
            _ => find_trails(bitmap, search_num + 1, rowpos, colpos + 1),
        };

        let mut hs = HashSet::new();

        trail_unwrapper(&mut hs, upval, upcoord);
        trail_unwrapper(&mut hs, downval, downcoord);
        trail_unwrapper(&mut hs, leftval, leftcoord);
        trail_unwrapper(&mut hs, rightval, rightcoord);

        return (upval + downval + leftval + rightval, Some(hs));
    }

    return (0, None);
}

fn main() {
    let fileread = fs::read_to_string("inputs/day10.txt").expect("Unable to read file as string");

    let bitmap = fileread
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch.to_digit(10) {
                    Some(x) => x as i32,
                    _ => 999,
                })
                .collect_vec()
        })
        .collect_vec();

    let trailheads = bitmap
        .iter()
        .enumerate()
        .map(|(row, vec)| {
            vec.iter()
                .enumerate()
                .map(|(col, ch)| match ch {
                    0 => find_trails(&bitmap, 0, row, col).0,
                    _ => 0,
                })
                .collect_vec()
        })
        .collect_vec();

    trailheads.iter().for_each(|line| {
        line.iter().for_each(|ch| {
            print!("{}", ch);
        });
        println!();
    });

    // For Part B, need cleaner code instead of summing the different paths
    let part_a = trailheads
        .iter()
        .map(|line| line.iter().fold(0, |acc, val| acc + val))
        .fold(0, |acc, val| acc + val);

    println!("Part A Trailheads: {}", part_a);
}
