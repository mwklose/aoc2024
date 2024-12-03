use std::fs;

fn meets_criteria(y: Vec<i32>) -> bool {
    let pairwise = (1..(y.len()))
        .map(|idx| (y[idx] - y[idx - 1]))
        .collect::<Vec<i32>>();

    let criteria_met = (pairwise.iter().all(|v| v.is_positive())
        || pairwise.iter().all(|v| v.is_negative()))
        && pairwise.iter().all(|v| (v.abs() >= 1) && (v.abs() <= 3));

    criteria_met
}

fn main() {
    let fileread = fs::read_to_string("inputs/day2.txt").expect("Unable to read file as string");

    let result = fileread.lines().into_iter();

    let count_criteria = result.map(|line| {
        let y = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        meets_criteria(y)
    });

    let valid_count = count_criteria.filter(|&x| x).count();
    println!("{}", valid_count);

    let part_2b = fileread
        .lines()
        .into_iter()
        .map(|line| {
            let y = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();

            (0..(y.len()))
                .map(|idx| {
                    let (lhs, rhs) = y.split_at(idx);
                    let idx_removed = [&lhs[..], &rhs[1..rhs.len()]].concat();

                    meets_criteria(idx_removed)
                })
                .any(|x| x)
        })
        .filter(|&x| x)
        .count();

    println!("{}", part_2b);
}
