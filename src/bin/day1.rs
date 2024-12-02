use std::fs;

fn main() {
    let mut lhs_vec = Vec::new();
    let mut rhs_vec = Vec::new();

    let fileread = fs::read_to_string("inputs/day1.txt").expect("Unable to read file as string");

    let result = fileread.lines().into_iter();

    for line in result {
        let mut y = line.split_whitespace();

        lhs_vec.push(y.next().unwrap().parse::<i64>().unwrap());
        rhs_vec.push(y.next().unwrap().parse::<i64>().unwrap());
    }

    lhs_vec.sort();
    rhs_vec.sort();

    let final_result = lhs_vec
        .iter()
        .zip(rhs_vec.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());

    println!("{}", final_result);

    let part_1b = lhs_vec
        .iter()
        .map(|x| x * rhs_vec.iter().filter(|y| *y == x).count() as i64)
        .fold(0, |acc, l| acc + l);

    println!("{}", part_1b);
}
