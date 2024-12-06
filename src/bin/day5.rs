use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Not,
};

fn main() {
    let fileread = fs::read_to_string("inputs/day5.txt").expect("Unable to read file as string");

    let (page_rules, update_rules) = fileread.split_once("\n\n").unwrap();

    // TODO: handle page rules and graph construction
    //
    let mut idx_hash = HashMap::new();

    for line in page_rules.lines() {
        let (lhs, rhs) = line.split_once("|").unwrap();
        let lhs_int = lhs.parse::<i32>().unwrap();
        let rhs_int = rhs.parse::<i32>().unwrap();

        // Insert into hashmap to keep track of connections
        let running_hashset = idx_hash.entry(lhs_int).or_insert(HashSet::new());
        running_hashset.insert(rhs_int);
    }

    let mut running_count = 0;
    let mut running_bcount = 0;
    for line in update_rules.lines() {
        let num_vec = line
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let default_hashset = &HashSet::<i32>::new();
        let num_vec_hash_sets = num_vec
            .clone()
            .into_iter()
            .map(|x| idx_hash.get(&x).unwrap_or(default_hashset))
            .collect::<Vec<&HashSet<i32>>>();

        let broke_rule = num_vec
            .iter()
            .enumerate()
            .map(|(idx, val)| {
                let (_, rhs) = num_vec_hash_sets.split_at(idx);

                rhs.iter().filter(|x| x.contains(val)).count()
            })
            .any(|count| count > 0);

        if !broke_rule {
            running_count += num_vec.iter().nth(num_vec.len() / 2).unwrap();
        }

        // If at this point, then broke rule, so need to correct.
        // But how?
        //

        let mut reordered_vec = Vec::<i32>::new();

        for (i, numb) in num_vec.iter().enumerate() {
            println!("ReorderedVec: {:?} inserting {}", reordered_vec, numb);
            if reordered_vec.len() == 0 {
                reordered_vec.insert(0, *numb);
                continue;
            }

            let insert_index: i32 = reordered_vec
                .iter()
                .map(|val| {
                    num_vec_hash_sets
                        .get(i)
                        .unwrap_or(&&default_hashset)
                        .contains(val)
                        .not()
                })
                .fold(0, |acc, x| acc + x as i32);

            reordered_vec.insert(insert_index as usize, *numb);
        }
        if broke_rule {
            running_bcount += reordered_vec.get(reordered_vec.len() / 2).unwrap();
        }

        println!("Running Count: {}", running_count);
        println!("Reordered Count: {}", running_bcount)
    }
}
