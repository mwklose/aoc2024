use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
enum GuardDirection {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn turn_right(gd: GuardDirection) -> GuardDirection {
    match gd {
        GuardDirection::UP => GuardDirection::RIGHT,
        GuardDirection::RIGHT => GuardDirection::DOWN,
        GuardDirection::DOWN => GuardDirection::LEFT,
        GuardDirection::LEFT => GuardDirection::UP,
    }
}

fn peek_next_step(
    bitmap: &Vec<Vec<i32>>,
    guard_dir: &GuardDirection,
    guard_row: usize,
    guard_col: usize,
) -> (bool, usize, usize) {
    if *guard_dir == GuardDirection::UP && guard_row == 0 {
        return (false, guard_row, guard_col);
    }

    if *guard_dir == GuardDirection::LEFT && guard_col == 0 {
        return (false, guard_row, guard_col);
    }

    let max_row = bitmap.len() - 1;
    if *guard_dir == GuardDirection::DOWN && guard_row == max_row {
        return (false, guard_row, guard_col);
    }

    let max_col = bitmap[0].len() - 1;
    if *guard_dir == GuardDirection::RIGHT && guard_col == max_col {
        return (false, guard_row, guard_col);
    }

    return match guard_dir {
        GuardDirection::UP => (true, guard_row - 1, guard_col),
        GuardDirection::DOWN => (true, guard_row + 1, guard_col),
        GuardDirection::LEFT => (true, guard_row, guard_col - 1),
        GuardDirection::RIGHT => (true, guard_row, guard_col + 1),
    };
}

fn helper_map(
    bitmap: &Vec<Vec<i32>>,
    mut visited_set: HashSet<(usize, usize)>,
    mut obstacle_set: HashSet<(GuardDirection, usize, usize)>,
    guard_pos: (usize, usize),
    guard_dir: GuardDirection,
) -> (
    bool,
    bool,
    HashSet<(usize, usize)>,
    HashSet<(GuardDirection, usize, usize)>,
    (usize, usize),
    GuardDirection,
) {
    let (guard_row, guard_col) = guard_pos;

    let (has_next_step, guard_peek_row, guard_peek_col) =
        peek_next_step(bitmap, &guard_dir, guard_row, guard_col);

    if !has_next_step {
        return (true, false, visited_set, obstacle_set, guard_pos, guard_dir);
    }

    if bitmap[guard_peek_row][guard_peek_col] == 1 {
        let new_insertion = obstacle_set.insert((guard_dir, guard_peek_row, guard_peek_col));

        if !new_insertion {
            println!(
                "Cycle Detected; with {:?} at {}, {}",
                guard_dir, guard_peek_row, guard_peek_col
            );

            return (true, true, visited_set, obstacle_set, guard_pos, guard_dir);
        }

        let new_guard_dir = turn_right(guard_dir);
        return (
            false,
            false,
            visited_set,
            obstacle_set,
            guard_pos,
            new_guard_dir,
        );
    }

    let next_guard_pos = (guard_peek_row, guard_peek_col);
    visited_set.insert(next_guard_pos);

    return (
        false,
        false,
        visited_set,
        obstacle_set,
        next_guard_pos,
        guard_dir,
    );
}

fn main() {
    let fileread = fs::read_to_string("inputs/day6.txt").expect("Unable to read file as string");

    let mut starting_point: (usize, usize) = (999, 999);
    let mut bitmap: Vec<Vec<i32>> = Vec::new();
    for (row, line) in fileread.lines().enumerate() {
        if line.contains("^") {
            starting_point = (row, line.find("^").unwrap());
        }

        bitmap.insert(
            row,
            line.chars()
                .map(|point| match point {
                    '#' => 1,
                    _ => 0,
                })
                .collect::<Vec<i32>>(),
        );
    }
    let first_rows = bitmap[0].len();
    assert!(bitmap
        .iter()
        .map(|line| line.len())
        .all(|val| val == first_rows));

    assert!(starting_point != (999, 999));
    let mut running_point = starting_point.clone();
    let mut num_iterations = 0;
    let mut completed_journey = false;
    let mut cycle_detected = false;

    let mut visited_set = HashSet::new();
    visited_set.insert(starting_point);

    let mut obstacle_set = HashSet::new();

    let mut guard_dir = GuardDirection::UP;
    while !completed_journey && num_iterations < 10000 {
        (
            completed_journey,
            cycle_detected,
            visited_set,
            obstacle_set,
            running_point,
            guard_dir,
        ) = helper_map(&bitmap, visited_set, obstacle_set, running_point, guard_dir);

        num_iterations += 1;
    }

    if cycle_detected {
        println!("Cycle detected.");
    }
    println!("{} in {}", visited_set.len(), num_iterations);

    // Part B - only need to check points surrounding the current path, which is much better than checking all positions

    let mut possible_obstruction_set = HashSet::new();
    let max_rows = bitmap.len();
    let max_cols = bitmap[0].len();

    for (visit_row, visit_col) in visited_set {
        if visit_row != 0 {
            possible_obstruction_set.insert((visit_row - 1, visit_col));
        }

        if visit_row != max_rows {
            possible_obstruction_set.insert((visit_row + 1, visit_col));
        }

        if visit_col != 0 {
            possible_obstruction_set.insert((visit_row, visit_col - 1));
        }

        if visit_col != max_cols {
            possible_obstruction_set.insert((visit_row, visit_col + 1));
        }
    }

    possible_obstruction_set.remove(&starting_point);

    println!("{}", possible_obstruction_set.len());

    let result_6b = possible_obstruction_set
        .iter()
        .filter(|(new_row, new_col)| {
            let mut running_point = starting_point.clone();
            let mut adjusted_bitmap = bitmap.clone();
            adjusted_bitmap[*new_row][*new_col] = 1;

            let mut num_iterations = 0;
            let mut completed_journey = false;
            let mut cycle_detected = false;

            let mut vset = HashSet::new();
            vset.insert(starting_point);

            let mut oset = HashSet::new();

            let mut guard_dir = GuardDirection::UP;
            while !completed_journey && num_iterations < 20000 {
                (
                    completed_journey,
                    cycle_detected,
                    vset,
                    oset,
                    running_point,
                    guard_dir,
                ) = helper_map(&adjusted_bitmap, vset, oset, running_point, guard_dir);

                num_iterations += 1;
            }

            println!(
                "({},{}): Completed:{}, Cycle:{}, NumIters:{}",
                new_row, new_col, completed_journey, cycle_detected, num_iterations
            );
            cycle_detected
        })
        .count();

    println!("Possible obstructions to add: {}", result_6b);
}
