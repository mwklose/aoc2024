use std::fs;

fn main() {
    let fileread = fs::read_to_string("inputs/day2.txt").expect("Unable to read file as string");

    let result = fileread.lines().into_iter();

    let count_criteria = result
        .map(|line| {
            let y = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();

            let mut pairwise = (1..5)
                .map(|idx| (y[idx] - y[idx - 1]))
                .collect::<Vec<i32>>();

            let meets_criteria = (pairwise.all(|v| v.is_positive())
                || pairwise.all(|v| v.is_negative()))
                && pairwise.all(|v| (v.abs() >= 1) && (v.abs() <= 3));

            meets_criteria
        })
        .filter(|&x| x)
        .count();

    println!("{}", count_criteria);
}
