use std::fs;

use nalgebra::{Matrix2, Matrix2x1};
use regex::Regex;

fn main() {
    let fileread = fs::read_to_string("inputs/day13.txt").expect("Unable to read file as string");

    let mut lines = fileread.lines();
    let re = Regex::new("Button .: X\\+(?<first>\\d+), Y\\+(?<second>\\d+)").unwrap();
    let re_prize = Regex::new("Prize: X=(?<first>\\d+), Y=(?<second>\\d+)").unwrap();

    let mut button_pushes: Vec<(f64, f64)> = Vec::new();
    let mut button_costs: Vec<f64> = Vec::new();
    let mut bigbutton_pushes: Vec<(f64, f64)> = Vec::new();
    let mut bigbutton_costs: Vec<f64> = Vec::new();

    while let (Some(l1), Some(l2), Some(l3)) = (lines.next(), lines.next(), lines.next()) {
        let (a_x, a_y) = match re.captures(l1) {
            Some(x) => (
                x["first"].parse::<f64>().unwrap(),
                x["second"].parse::<f64>().unwrap(),
            ),
            _ => panic!("No captures found?"),
        };

        let (b_x, b_y) = match re.captures(l2) {
            Some(x) => (
                x["first"].parse::<f64>().unwrap(),
                x["second"].parse::<f64>().unwrap(),
            ),
            _ => panic!("No captures found?"),
        };

        let (c_x, c_y) = match re_prize.captures(l3) {
            Some(x) => (
                x["first"].parse::<f64>().unwrap(),
                x["second"].parse::<f64>().unwrap(),
            ),
            _ => panic!("No captures found?"),
        };

        let shift = 10000000000000.0;
        let (cbig_x, cbig_y) = (c_x + shift, c_y + shift);
        // TODO: nalgebra to make matrices, and work from there to solve.
        let coords = Matrix2::new(a_x, b_x, a_y, b_y);
        let target = Matrix2x1::new(c_x, c_y);
        let big_target = Matrix2x1::new(cbig_x, cbig_y);
        let costs = Matrix2x1::new(3.0, 1.0);

        let result = coords.lu().solve(&target).unwrap();
        let big_result = coords.lu().solve(&big_target).unwrap();

        if result
            .iter()
            .filter(|it| **it >= 0.0 && (**it - it.round()).abs() < 1e-5)
            .count()
            == 2
        {
            let rounded_x = result.x.round();
            let rounded_y = result.y.round();
            let cost = (result.transpose() * costs).x;

            button_pushes.push((rounded_x, rounded_y));
            button_costs.push(cost);
        }

        if big_result
            .iter()
            .filter(|it| **it >= 0.0 && (**it - it.round()).abs() < 1e-4)
            .count()
            == 2
        {
            let rounded_x = big_result.x.round();
            let rounded_y = big_result.y.round();
            let cost = (big_result.transpose() * costs).x;
            println!(
                "{}x{} = {} ?= {}",
                rounded_x,
                rounded_y,
                rounded_x * 3.0 + rounded_y,
                cost
            );

            bigbutton_pushes.push((rounded_x, rounded_y));
            bigbutton_costs.push(cost);
        }

        lines.next();
    }

    let result_a: f64 = button_pushes
        .iter()
        .fold(0., |acc, (a, b)| acc + 3. * a + 1. * b);

    let result_mult: f64 = button_costs.iter().fold(0., |acc, x| acc + x);
    println!("Tokens part 1: {} ?= {}", result_a, result_mult);

    let result_b: f64 = bigbutton_pushes
        .iter()
        .fold(0., |acc, (a, b)| acc + 3. * a + 1. * b);
    let result_b_mult = bigbutton_costs.iter().fold(0., |acc, x| acc + x);

    // Not 54814825589439
    // Trying 85644161121698
    println!("Tokens part 2: {} ?= {}", result_b, result_b_mult);
}
