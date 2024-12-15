use std::{
    collections::{HashMap, HashSet},
    fs,
};

use regex::Regex;

fn simulate_steps(
    xpos: i32,
    ypos: i32,
    xvel: i32,
    yvel: i32,
    num_steps: i32,
    max_rows: i32,
    max_cols: i32,
) -> (i32, i32) {
    let sim_x = (xpos + xvel * num_steps).rem_euclid(max_rows);
    let sim_y = (ypos + yvel * num_steps).rem_euclid(max_cols);
    return (sim_x, sim_y);
}

fn find_quadrant(pos: (i32, i32), max_rows: i32, max_cols: i32) -> (i32, i32, i32, i32) {
    let horizontal_boundary = max_rows / 2;
    let vertical_boundary = max_cols / 2;

    if pos.0 < horizontal_boundary && pos.1 < vertical_boundary {
        return (1, 0, 0, 0);
    } else if pos.0 > horizontal_boundary && pos.1 < vertical_boundary {
        return (0, 1, 0, 0);
    } else if pos.0 < horizontal_boundary && pos.1 > vertical_boundary {
        return (0, 0, 1, 0);
    } else if pos.0 > horizontal_boundary && pos.1 > vertical_boundary {
        return (0, 0, 0, 1);
    }

    return (0, 0, 0, 0);
}
fn main() {
    let fileread = fs::read_to_string("inputs/day14.txt").expect("Unable to read file as string");

    let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();

    let num_rows = 101;
    let num_cols = 103;

    let result = fileread
        .lines()
        .map(|line| match re.captures(line) {
            Some(x) => simulate_steps(
                x["px"].parse::<i32>().unwrap(),
                x["py"].parse::<i32>().unwrap(),
                x["vx"].parse::<i32>().unwrap(),
                x["vy"].parse::<i32>().unwrap(),
                100,
                num_rows,
                num_cols,
            ),
            _ => panic!("Unable to capture everything???"),
        })
        .fold((0, 0, 0, 0, 1), |(a1, a2, a3, a4, _), pos| {
            let (q1, q2, q3, q4) = find_quadrant(pos, num_rows, num_cols);
            (
                a1 + q1,
                a2 + q2,
                a3 + q3,
                a4 + q4,
                (a1 + q1) * (a2 + q2) * (a3 + q3) * (a4 + q4),
            )
        });

    println!("Result: {:?}", result);

    // TODO: christmas tree shape?
    let mut wobot_hm: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    fileread
        .lines()
        .map(|line| match re.captures(line) {
            Some(x) => (
                x["px"].parse::<i32>().unwrap(),
                x["py"].parse::<i32>().unwrap(),
                x["vx"].parse::<i32>().unwrap(),
                x["vy"].parse::<i32>().unwrap(),
            ),
            _ => panic!("Unable to capture everything?????"),
        })
        .for_each(|(px, py, vx, vy)| {
            wobot_hm.insert((px, py), (vx, vy));
        });

    let n_sims = 10500;
    for sim in 0..n_sims {
        println!("\n----{}----\n", sim);
        let mut sim_hm: HashSet<(i32, i32)> = HashSet::new();
        wobot_hm.iter().for_each(|((kx, ky), (vx, vy))| {
            sim_hm.insert(simulate_steps(*kx, *ky, *vx, *vy, sim, num_rows, num_cols));
        });
        for row in 0..num_rows {
            for col in 0..num_cols {
                match sim_hm.get(&(row, col)) {
                    Some(_) => print!("A"),
                    _ => print!("."),
                }
            }
            println!();
        }
    }
}
