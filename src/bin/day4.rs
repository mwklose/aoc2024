use std::fs;

fn char_array_match(cmap: &Vec<Vec<char>>, coords: &Vec<(i32, i32)>, match_str: &str) -> i32 {
    for (i, (row, column)) in coords.iter().enumerate() {
        let coord_row = *row as usize;
        let coord_column = *column as usize;
        if coord_row >= cmap.len() {
            return 0;
        }

        if coord_column >= cmap[coord_row].len() {
            return 0;
        }

        if cmap[coord_row][coord_column] != match_str.chars().nth(i).unwrap() {
            return 0;
        }
    }

    return 1;
}

fn part_4b_match(cmap: &Vec<Vec<char>>, start_coords: (usize, usize)) -> i32 {
    // TODO: complete
    let words = vec!["SAM", "MAS"];
    let (row, column) = start_coords;

    let decreasing: i32 = words
        .iter()
        .map(|word| {
            char_array_match(
                cmap,
                &(0..word.len())
                    .map(|idx| ((row + idx) as i32, (column + idx) as i32))
                    .collect::<Vec<(i32, i32)>>(),
                word,
            )
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    let increasing: i32 = words
        .iter()
        .map(|word| {
            char_array_match(
                cmap,
                &(0..word.len())
                    .map(|idx| ((row + idx) as i32, (column + 2 - idx) as i32))
                    .collect::<Vec<(i32, i32)>>(),
                word,
            )
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    return decreasing * increasing;
}

fn main() {
    let fileread = fs::read_to_string("inputs/day4.txt").expect("Unable to read file as string");

    let char_array = fileread
        .lines()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let char_array_columns = char_array[0].len() as i32;
    let char_array_rows = char_array.len() as i32;
    let mut running_total = 0;
    let mut part_4b_total = 0;
    for row in 0..(char_array_rows) {
        for column in 0..(char_array_columns) {
            // TODO: check for values here
            let words = vec!["XMAS", "SAMX"];
            let horizontal: i32 = words
                .iter()
                .map(|word| {
                    char_array_match(
                        &char_array,
                        &(0..word.len() as i32)
                            .map(|idx| (row, column + idx))
                            .collect::<Vec<(i32, i32)>>(),
                        word,
                    )
                })
                .collect::<Vec<i32>>()
                .iter()
                .sum();
            let vertical: i32 = words
                .iter()
                .map(|word| {
                    char_array_match(
                        &char_array,
                        &(0..word.len() as i32)
                            .map(|idx| (row + idx, column))
                            .collect::<Vec<(i32, i32)>>(),
                        word,
                    )
                })
                .collect::<Vec<i32>>()
                .iter()
                .sum();
            let decreasing: i32 = words
                .iter()
                .map(|word| {
                    char_array_match(
                        &char_array,
                        &(0..word.len() as i32)
                            .map(|idx| (row + idx, column + idx))
                            .collect::<Vec<(i32, i32)>>(),
                        word,
                    )
                })
                .collect::<Vec<i32>>()
                .iter()
                .sum();
            let increasing: i32 = words
                .iter()
                .map(|word| {
                    char_array_match(
                        &char_array,
                        &(0..word.len() as i32)
                            .map(|idx| (row + idx as i32, column - idx as i32))
                            .collect::<Vec<(i32, i32)>>(),
                        word,
                    )
                })
                .collect::<Vec<i32>>()
                .iter()
                .sum();

            running_total += horizontal;
            running_total += vertical;
            running_total += decreasing;
            running_total += increasing;

            part_4b_total += part_4b_match(
                &char_array,
                (row.try_into().unwrap(), column.try_into().unwrap()),
            );
        }

        println!("Row {}: {}, XMAS: {}", row, running_total, part_4b_total);
    }

    println!("Overall total: {}, XMAS: {}", running_total, part_4b_total);
}
