use std::{collections::VecDeque, fs};

use itertools::Itertools;

struct PlaceHolder {
    is_empty: bool,
    id: usize,
}
#[derive(PartialEq, Copy, Clone, Debug)]
struct CapPlaceHolder {
    is_empty: bool,
    id: usize,
    count: u32,
    cumsum: u32,
}

fn main() {
    let fileread = fs::read_to_string("inputs/day9.txt").expect("Unable to read file as string");

    // Assummption: file block contains IDs, a single slot can have a number greater than 9

    let mut queue: VecDeque<PlaceHolder> = VecDeque::new();

    fileread.chars().enumerate().for_each(|(idx, ch)| {
        match (ch.to_digit(10), idx % 2 == 0) {
            (Some(x), true) => {
                (0..x).into_iter().for_each(|_| {
                    queue.push_back(PlaceHolder {
                        is_empty: false,
                        id: idx / 2,
                    })
                });
            }
            (Some(x), false) => {
                (0..x).into_iter().for_each(|_| {
                    queue.push_back(PlaceHolder {
                        is_empty: true,
                        id: 99999,
                    });
                });
            }
            (None, _) => {}
        };
    });

    for ph in queue.iter() {
        match ph.is_empty {
            true => print!("."),
            false => print!("{}", ph.id),
        }
    }
    println!();

    // Then, alternate between taking from front and back to get density of space
    let mut output_queue: VecDeque<usize> = VecDeque::new();
    while queue.len() > 0 {
        let front_obj = queue.pop_front().unwrap();
        // TODO: logic for inserting from back once ready.

        if front_obj.is_empty {
            while queue.back().unwrap().is_empty {
                queue.pop_back();
            }

            let back_obj = queue.pop_back().unwrap();

            output_queue.push_back(back_obj.id);
        } else {
            output_queue.push_back(front_obj.id);
        }
    }

    let queue_result = output_queue
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, val)| acc + idx * val);

    println!("Result Part A: {}", queue_result);

    // Part B - need to process with different approachmut .
    //
    let mut queue_b: VecDeque<CapPlaceHolder> = VecDeque::new();
    let mut cumsum = 0;
    fileread
        .chars()
        .enumerate()
        .for_each(|(idx, ch)| match ch.to_digit(10) {
            Some(x) => {
                queue_b.push_back(CapPlaceHolder {
                    is_empty: idx % 2 == 1,
                    id: (idx + 1) / 2,
                    count: x,
                    cumsum,
                });
                cumsum += x;
            }
            _ => {}
        });

    let mut running_order = queue_b.clone();
    let mut helper_queue = VecDeque::new();
    let to_check = queue_b
        .iter()
        .filter(|val| !val.is_empty)
        .rev()
        .collect_vec();

    for item in to_check.iter() {
        let mut running_cumsum = 0;

        while running_order.len() > 0 {
            let current_check = running_order.pop_front().unwrap();
            running_cumsum += current_check.count;

            if running_cumsum > item.cumsum {
                helper_queue.push_back(current_check);
                continue;
            }

            if !current_check.is_empty {
                helper_queue.push_back(current_check);
                continue;
            }

            if current_check.count < item.count {
                helper_queue.push_back(current_check);
                continue;
            }

            if current_check.count == item.count {
                helper_queue.push_back(**item);
            } else {
                let blank_length = current_check.count - item.count;
                helper_queue.push_back(**item);
                helper_queue.push_back(CapPlaceHolder {
                    is_empty: true,
                    id: item.id,
                    count: blank_length,
                    cumsum: 0,
                });
            }

            let pp = running_order.iter().rposition(|x| x.id == item.id).unwrap();
            let mut rhs = running_order.split_off(pp);
            rhs.retain(|x| x.id != item.id);

            helper_queue.append(&mut running_order);
            helper_queue.push_back(CapPlaceHolder {
                is_empty: true,
                id: item.id,
                count: item.count,
                cumsum: 0,
            });
            helper_queue.append(&mut rhs);
            // TODO: something up with appending remainder to running order?
            //
            // Issue with 8 or 6 - no slot for it, so need to put at the end.
        }

        running_order = helper_queue.clone();

        helper_queue.clear();
    }

    let mut result_9b = VecDeque::new();

    for item in running_order.iter() {
        match item.is_empty {
            false => {
                (0..item.count).for_each(|_| result_9b.push_back(item.id));
            }
            _ => (0..item.count).for_each(|_| result_9b.push_back(0)),
        }
    }
    let queue_result = result_9b
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, val)| acc + idx * val);

    println!("Result Part B: {}", queue_result);
}
