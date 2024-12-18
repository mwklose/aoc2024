use std::{collections::VecDeque, fs};

use itertools::Itertools;
use regex::Regex;

fn find_combo(combo: i8, reg_a: i64, reg_b: i64, reg_c: i64) -> i64 {
    match combo {
        0 | 1 | 2 | 3 => combo as i64,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => panic!("Invalid combo argument"),
    }
}

fn adv(
    instruction_ptr: usize,
    operand: i8,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
) -> (Option<i64>, usize, i64, i64, i64) {
    let combo = find_combo(operand, reg_a, reg_b, reg_c);

    return (
        None,
        instruction_ptr + 2,
        reg_a / (2_i64.pow(combo as u32)),
        reg_b,
        reg_c,
    );
}

fn bxl(
    instruction_ptr: usize,
    operand: i8,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
) -> (Option<i64>, usize, i64, i64, i64) {
    return (
        None,
        instruction_ptr + 2,
        reg_a,
        reg_b ^ operand as i64,
        reg_c,
    );
}

fn bst(
    instruction_ptr: usize,
    operand: i8,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
) -> (Option<i64>, usize, i64, i64, i64) {
    let combo = find_combo(operand, reg_a, reg_b, reg_c);
    return (None, instruction_ptr + 2, reg_a, combo % 8, reg_c);
}

fn jnz(
    instruction_ptr: usize,
    operand: i8,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
) -> (Option<i64>, usize, i64, i64, i64) {
    if reg_a == 0 {
        return (None, instruction_ptr + 2, reg_a, reg_b, reg_c);
    }

    return (None, operand as usize, reg_a, reg_b, reg_c);
}

fn bxc(
    instruction_ptr: usize,
    _operand: i8,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
) -> (Option<i64>, usize, i64, i64, i64) {
    return (None, instruction_ptr + 2, reg_a, reg_b ^ reg_c, reg_c);
}

fn out(
    instruction_ptr: usize,
    operand: i8,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
) -> (Option<i64>, usize, i64, i64, i64) {
    let combo = find_combo(operand, reg_a, reg_b, reg_c);

    return (Some(combo % 8), instruction_ptr + 2, reg_a, reg_b, reg_c);
}

fn bdv(
    instruction_ptr: usize,
    operand: i8,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
) -> (Option<i64>, usize, i64, i64, i64) {
    let combo = find_combo(operand, reg_a, reg_b, reg_c);

    return (
        None,
        instruction_ptr + 2,
        reg_a,
        reg_a / (2_i64.pow(combo as u32)),
        reg_c,
    );
}

fn cdv(
    instruction_ptr: usize,
    operand: i8,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
) -> (Option<i64>, usize, i64, i64, i64) {
    let combo = find_combo(operand, reg_a, reg_b, reg_c);

    return (
        None,
        instruction_ptr + 2,
        reg_a,
        reg_b,
        reg_a / (2_i64.pow(combo as u32)),
    );
}

fn perform_instruction(
    instruction_ptr: usize,
    program: &Vec<i8>,
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
) -> (Option<i64>, usize, i64, i64, i64) {
    let instruction = program.get(instruction_ptr).unwrap();
    let operand = program.get(instruction_ptr + 1).unwrap();

    match instruction {
        // TODO: complete implementations here
        0 => adv(instruction_ptr, *operand, reg_a, reg_b, reg_c),
        1 => bxl(instruction_ptr, *operand, reg_a, reg_b, reg_c),
        2 => bst(instruction_ptr, *operand, reg_a, reg_b, reg_c),
        3 => jnz(instruction_ptr, *operand, reg_a, reg_b, reg_c),
        4 => bxc(instruction_ptr, *operand, reg_a, reg_b, reg_c),
        5 => out(instruction_ptr, *operand, reg_a, reg_b, reg_c),
        6 => bdv(instruction_ptr, *operand, reg_a, reg_b, reg_c),
        7 => cdv(instruction_ptr, *operand, reg_a, reg_b, reg_c),
        _ => panic!("Cannot perform instruction."),
    }
}

fn simulate_register(prog_vec: &Vec<i8>, init_reg_a: i64) -> Vec<i64> {
    let mut instruction_ptr = 0;

    let mut reg_a = init_reg_a;
    let mut reg_b = 0;
    let mut reg_c = 0;

    let mut out_vec: Vec<i64> = Vec::new();

    while instruction_ptr < prog_vec.len() {
        let result: Option<i64>;
        (result, instruction_ptr, reg_a, reg_b, reg_c) =
            perform_instruction(instruction_ptr, &prog_vec, reg_a, reg_b, reg_c);

        match result {
            Some(x) => {
                out_vec.push(x);
            }
            None => {}
        }
    }

    return out_vec;
}

fn running_rev_match(prog_vec: &Vec<i8>, out_vec: &Vec<i64>) -> bool {
    out_vec
        .iter()
        .rev()
        .enumerate()
        .all(|(idx, val)| *val == *prog_vec.get(prog_vec.len() - idx - 1).unwrap_or(&0_i8) as i64)
}

fn main() {
    let fileread = fs::read_to_string("inputs/day17.txt").expect("Unable to read file as string");

    let re = Regex::new(r"Register A: (?<reg_a>\d+)\nRegister B: (?<reg_b>\d+)\nRegister C: (?<reg_c>\d+)\n\nProgram: (?<prog>[\d,]+)").unwrap();

    let (mut reg_a, mut reg_b, mut reg_c, prog_vec) = match re.captures(&fileread) {
        Some(x) => (
            x["reg_a"].parse::<i64>().unwrap(),
            x["reg_b"].parse::<i64>().unwrap(),
            x["reg_c"].parse::<i64>().unwrap(),
            x["prog"]
                .split(",")
                .map(|x| x.parse::<i8>().unwrap())
                .collect_vec(),
        ),
        _ => panic!("Unable to parse file."),
    };

    let mut instruction_ptr = 0;

    while instruction_ptr < prog_vec.len() {
        // TODO: actually run the program
        let result: Option<i64>;
        (result, instruction_ptr, reg_a, reg_b, reg_c) =
            perform_instruction(instruction_ptr, &prog_vec, reg_a, reg_b, reg_c);

        match result {
            Some(x) => print!("{},", x),
            None => {}
        };
    }

    println!();

    // Testing:
    //Program: 2,4,1,2,7,5,1,3,4,3,5,5,0,3,3,0
    // Observations: highest bits indicate lower values

    // This constant is good for the final 4 digits

    let mut init_a_deque: VecDeque<i64> = VecDeque::new();
    let mut candidate_vec: Vec<i64> = Vec::new();
    init_a_deque.push_back(0b1000011101);

    while init_a_deque.len() > 0 {
        let init_a = init_a_deque.pop_front().unwrap();
        let possible_results = (0..8_i64.pow(2))
            .filter(|val| {
                let result_vec = simulate_register(&prog_vec, (init_a << 6) | val);
                running_rev_match(&prog_vec, &result_vec)
            })
            .collect_vec();

        println!("Candidates for {} (0b{:b}):", init_a, init_a);
        for pr in possible_results.iter() {
            let test_init_a = (init_a << 6) | pr;

            init_a_deque.push_back(test_init_a);
            println!(
                "\t{} (0b{:b}) -> {:?}",
                test_init_a,
                test_init_a,
                simulate_register(&prog_vec, test_init_a)
            );

            if simulate_register(&prog_vec, test_init_a).len() == prog_vec.len() {
                println!("\t\tFound a match!! {}", test_init_a);
                candidate_vec.push(test_init_a);
            }
        }
    }

    println!("Minimum match: {}", candidate_vec.iter().min().unwrap());
}
