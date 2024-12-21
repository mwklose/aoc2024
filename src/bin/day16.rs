use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use petgraph::{
    algo::{all_simple_paths, dijkstra},
    graph::NodeIndex,
    Graph,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn junction_directions(my_direction: &Direction) -> (Direction, Direction) {
    match my_direction {
        &Direction::NORTH | &Direction::SOUTH => (Direction::EAST, Direction::WEST),
        &Direction::EAST | &Direction::WEST => (Direction::NORTH, Direction::SOUTH),
    }
}

fn step_direction(
    bitmap: &Vec<Vec<char>>,
    start_loc: (usize, usize),
    direction: &Direction,
) -> (bool, (usize, usize)) {
    let max_rows = bitmap.len();
    let max_cols = bitmap[0].len();

    match direction {
        Direction::EAST => {
            if start_loc.1 + 1 >= max_cols || bitmap[start_loc.0][start_loc.1 + 1] == '#' {
                return (false, start_loc);
            }
            (true, (start_loc.0, start_loc.1 + 1))
        }
        Direction::NORTH => {
            if start_loc.0 == 0 || bitmap[start_loc.0 - 1][start_loc.1] == '#' {
                return (false, start_loc);
            }
            (true, (start_loc.0 - 1, start_loc.1))
        }
        Direction::WEST => {
            if start_loc.1 == 0 || bitmap[start_loc.0][start_loc.1 - 1] == '#' {
                return (false, start_loc);
            }
            (true, (start_loc.0, start_loc.1 - 1))
        }
        Direction::SOUTH => {
            if start_loc.0 + 1 >= max_rows || bitmap[start_loc.0 + 1][start_loc.1] == '#' {
                return (false, start_loc);
            }
            (true, (start_loc.0 + 1, start_loc.1))
        }
    }

    // Returns can step that direction, location of step if valid
}

fn go_direction(
    bitmap: &Vec<Vec<char>>,
    start_loc: (usize, usize),
    direction: Direction,
    turning: bool,
) -> (bool, i32, (usize, usize)) {
    let mut running_score = 0;
    // Check that valid starting point
    assert_ne!(bitmap[start_loc.0][start_loc.1], '#');

    // If turning, then need to add points
    if turning {
        running_score += 1000;
    }

    // Peek first direction
    let (can_go_direction, next_step) = step_direction(&bitmap, start_loc, &direction);

    if !can_go_direction {
        return (false, running_score, start_loc);
    }

    running_score += 1;

    return (true, running_score, next_step);
}

fn main() {
    let fileread = fs::read_to_string("inputs/day16.txt").expect("Unable to read file as string");

    // Approach:
    // Parse bitmap
    let mut bitmap: Vec<Vec<char>> = Vec::new();
    let mut end_node: (usize, usize) = (0, 0);
    let mut start_node: (usize, usize) = (0, 0);
    fileread.lines().enumerate().for_each(|(row, line)| {
        let mut line_vec = Vec::new();
        line.chars().enumerate().for_each(|(col, ch)| {
            if ch == 'E' {
                end_node = (row, col);
            }
            if ch == 'S' {
                start_node = (row, col);
            }
            line_vec.push(ch);
        });
        bitmap.push(line_vec);
    });

    // Create graph (nodes = junctions, start, end; edge_weight = scoring)
    let mut graph: Graph<((usize, usize), Direction), i32> = Graph::new();
    let mut processed_junctions: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut node_map: HashMap<(usize, usize), NodeIndex> = HashMap::new();
    let mut inverse_node_map: HashMap<NodeIndex, (usize, usize)> = HashMap::new();
    let mut process_queue: VecDeque<((usize, usize), Direction)> = VecDeque::new();

    // Initialize with 4 directions from starting node
    process_queue.push_back((start_node, Direction::EAST));

    while let Some((next_loc, next_dir)) = process_queue.pop_front() {
        if processed_junctions.contains(&(next_loc, next_dir)) {
            continue;
        }

        processed_junctions.insert((next_loc, next_dir));

        println!(
            "Processing junction {:?} {:?} -> {:?}",
            next_dir, next_loc, bitmap[next_loc.0][next_loc.1]
        );

        let (can_dir, score_dir, next_loc_dir) = go_direction(&bitmap, next_loc, next_dir, false);

        if can_dir && !processed_junctions.contains(&(next_loc_dir, next_dir)) {
            process_queue.push_front((next_loc_dir, next_dir));

            let nl = match node_map.get(&next_loc) {
                Some(&x) => x,
                _ => {
                    let gan = graph.add_node((next_loc, next_dir));
                    node_map.insert(next_loc, gan);
                    inverse_node_map.insert(gan, next_loc);
                    gan
                }
            };

            let nld = match node_map.get(&next_loc_dir) {
                Some(&x) => x,
                _ => {
                    let gan = graph.add_node((next_loc_dir, next_dir));
                    node_map.insert(next_loc_dir, gan);
                    inverse_node_map.insert(gan, next_loc_dir);
                    gan
                }
            };
            if !graph.contains_edge(nl, nld) {
                graph.update_edge(nl, nld, score_dir);
            }
        }

        let (ne, sw) = junction_directions(&next_dir);

        println!("NE={:?}; SW={:?}", ne, sw);
        let (can_ne, score_ne, next_loc_ne) = go_direction(&bitmap, next_loc, ne, true);

        if can_ne && !processed_junctions.contains(&(next_loc_ne, ne)) {
            process_queue.push_back((next_loc_ne, ne));
            assert!(score_ne > 1000);

            let nl = match node_map.get(&next_loc) {
                Some(&x) => x,
                _ => {
                    let gan = graph.add_node((next_loc, ne));
                    node_map.insert(next_loc, gan);
                    inverse_node_map.insert(gan, next_loc);
                    gan
                }
            };

            let nld = match node_map.get(&next_loc_ne) {
                Some(&x) => x,
                _ => {
                    let gan = graph.add_node((next_loc_ne, ne));
                    node_map.insert(next_loc_ne, gan);
                    inverse_node_map.insert(gan, next_loc_ne);
                    gan
                }
            };
            if !graph.contains_edge(nl, nld) {
                graph.update_edge(nl, nld, score_ne);
            }
        }

        let (can_sw, score_sw, next_loc_sw) = go_direction(&bitmap, next_loc, sw, true);

        if can_sw && !processed_junctions.contains(&(next_loc_sw, sw)) {
            assert!(score_sw > 1000);
            process_queue.push_back((next_loc_sw, sw));

            let nl = match node_map.get(&next_loc) {
                Some(&x) => x,
                _ => {
                    let gan = graph.add_node((next_loc, sw));
                    node_map.insert(next_loc, gan);
                    inverse_node_map.insert(gan, next_loc);
                    gan
                }
            };

            let nld = match node_map.get(&next_loc_sw) {
                Some(&x) => x,
                _ => {
                    let gan = graph.add_node((next_loc_sw, sw));
                    node_map.insert(next_loc_sw, gan);
                    inverse_node_map.insert(gan, next_loc_sw);
                    gan
                }
            };

            if !graph.contains_edge(nl, nld) {
                graph.update_edge(nl, nld, score_sw);
            }
        }
    }

    // Djikstra's

    let sn_index = match node_map.get(&start_node) {
        Some(&x) => x,
        _ => panic!("Starting node never reached?"),
    };

    let en_index = match node_map.get(&end_node) {
        Some(&x) => x,
        _ => panic!("Ending node never reached?"),
    };

    let result = dijkstra(&graph, sn_index, Some(en_index), |e| *e.weight());

    println!(
        "Part A:\n\tStart:{:?}\n\tEnd:{:?}\n\t{:?}",
        sn_index,
        en_index,
        result.get(&en_index)
    );

    // TODO: A* to find number of nodes, which must be equal across all implementations, otherwise, unequal scores

    let part_b = all_simple_paths::<Vec<_>, _>(&graph, sn_index, en_index, 0, None);

    for (i, path) in part_b.enumerate() {
        println!("{}: {:?}\n", i, path);
    }
}
