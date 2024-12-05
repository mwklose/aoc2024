use regex::Regex;

use std::fs;

fn main() {
    let fileread = fs::read_to_string("inputs/day3.txt").expect("Unable to read file as string");

    let result = fileread.lines().into_iter();

    let re = Regex::new(r"mul\((?<lhs>[[:digit:]]+),(?<rhs>[[:digit:]]+)\)").unwrap();
    let mut result_vec = Vec::new();

    result.for_each(|line| {
        re.captures_iter(line)
            .map(|x| x.extract())
            .for_each(|(_, [lhs, rhs])| {
                result_vec.push(lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap())
            });
    });

    println!("{}", result_vec.iter().fold(0, |acc, x| acc + x));

    let mut b_vec: Vec<i32> = Vec::new();

    fileread.split(r"do()").for_each(|dosplit| {
        println!("dosplit={}", dosplit);
        let lhs = dosplit.split(r"don't()").next().unwrap();

        re.captures_iter(lhs)
            .map(|x| x.extract())
            .for_each(|(mat, [l, r])| {
                println!("{} -> {};;;{}\n\n", mat, l, r);
                b_vec.push(l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap())
            });
    });

    println!("{}", b_vec.iter().fold(0, |acc, x| acc + x));
}
