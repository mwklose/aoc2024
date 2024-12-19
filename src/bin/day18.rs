use std::{collections::HashMap, fs};

use petgraph::{algo::dijkstra, graph::NodeIndex, prelude::StableDiGraph};

fn main() {
    let fileread = fs::read_to_string("inputs/day18.txt").expect("Unable to read file as string");

    // Create map of space
    let nrow = 71;
    let ncol = 71;
    let mut bitmap = [[0_i32; 71]; 71];
    let mut node_set: HashMap<(usize, usize), NodeIndex> = HashMap::new();
    let mut graph: StableDiGraph<(usize, usize), i32> = StableDiGraph::new();

    for row in 0..nrow {
        for col in 0..ncol {
            println!("Handling {},{}", row, col);
            let current_node = match node_set.get(&(row, col)) {
                Some(x) => *x,
                _ => {
                    let new_node = graph.add_node((row, col));
                    node_set.insert((row, col), new_node);
                    new_node
                }
            };

            if (row + 1) < nrow {
                let right_node = match node_set.get(&(row + 1, col)) {
                    Some(x) => *x,
                    _ => {
                        let new_node = graph.add_node((row + 1, col));
                        node_set.insert((row + 1, col), new_node);
                        new_node
                    }
                };

                graph.add_edge(current_node, right_node, 1);
            }

            if (col + 1) < ncol {
                let down_node = match node_set.get(&(row, col + 1)) {
                    Some(x) => *x,
                    _ => {
                        let new_node = graph.add_node((row, col + 1));
                        node_set.insert((row, col + 1), new_node);
                        new_node
                    }
                };

                graph.add_edge(current_node, down_node, 1);
            }

            if row != 0 {
                let up_node = match node_set.get(&(row - 1, col)) {
                    Some(x) => *x,
                    _ => {
                        let new_node = graph.add_node((row - 1, col));
                        node_set.insert((row - 1, col), new_node);
                        new_node
                    }
                };

                graph.add_edge(current_node, up_node, 1);
            }

            if col != 0 {
                let left_node = match node_set.get(&(row, col - 1)) {
                    Some(x) => *x,
                    _ => {
                        let new_node = graph.add_node((row, col - 1));
                        node_set.insert((row, col - 1), new_node);
                        new_node
                    }
                };

                graph.add_edge(current_node, left_node, 1);
            }
        }
    }
    println!("{:#?}", graph);

    println!(
        "Node 0,0: {:?};\n\tNode 70,70: {:?}",
        node_set.get(&(0, 0)),
        node_set.get(&(nrow - 1, ncol - 1))
    );
    for (linenum, line) in fileread.lines().enumerate() {
        if linenum >= 1024 {
            break;
        }
        let (lhs_int, rhs_int) = match line.split_once(",") {
            Some((lhs, rhs)) => (lhs.parse::<usize>().unwrap(), rhs.parse::<usize>().unwrap()),
            _ => panic!("Unable to split line"),
        };

        assert_eq!(
            graph
                .remove_node(*node_set.get(&(rhs_int, lhs_int)).unwrap())
                .unwrap(),
            (rhs_int, lhs_int)
        );

        bitmap[rhs_int][lhs_int] = 1;
    }

    // Path finding to get out of map
    // Isn't that just Djikstra, find distance between two nodes?
    let res = dijkstra(&graph, *node_set.get(&(0, 0)).unwrap(), None, |_| 1);

    let end_node = node_set.get(&(nrow - 1, ncol - 1)).unwrap();
    println!("Part A: {:?}", res.get(end_node));

    fileread.lines().enumerate().for_each(|(linenum, line)| {
        let (lhs_int, rhs_int) = match line.split_once(",") {
            Some((lhs, rhs)) => (lhs.parse::<usize>().unwrap(), rhs.parse::<usize>().unwrap()),
            _ => panic!("Unable to split line"),
        };

        graph.remove_node(*node_set.get(&(rhs_int, lhs_int)).unwrap());

        let end_node = node_set.get(&(nrow - 1, ncol - 1)).unwrap();
        let result = dijkstra(&graph, *node_set.get(&(0, 0)).unwrap(), None, |_| 1);

        println!("{}: {} -> {:?}", linenum, line, result.get(end_node));
    });
}
