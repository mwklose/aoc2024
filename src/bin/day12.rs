use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Region {
    items: HashSet<(usize, usize)>,
    charname: char,
    perimeter: i32,
    area: i32,
}

impl Region {
    fn new(row: usize, col: usize, charname: char) -> Region {
        let mut hs = HashSet::new();
        hs.insert((row, col));

        Region {
            items: hs,
            charname,
            perimeter: 4,
            area: 1,
        }
    }

    fn touches_point(&self, point_row: usize, point_col: usize) -> bool {
        self.items.iter().any(|item| {
            if point_row == item.0 && point_col == item.1 {
                panic!("Attempting to insert point that already exists");
            }

            let up = point_row > item.0 && point_col == item.1 && (point_row - item.0) == 1;
            let down = point_row < item.0 && point_col == item.1 && (item.0 - point_row) == 1;
            let left = point_row == item.0 && point_col > item.1 && (point_col - item.1) == 1;
            let right = point_row == item.0 && point_col < item.1 && (item.1 - point_col) == 1;
            up || down || left || right
        })
    }

    fn count_touches(hs: HashSet<(usize, usize)>, point_row: usize, point_col: usize) -> i32 {
        let touches_left = hs
            .iter()
            .filter(|item| point_row == item.0 && point_col > item.1 && (point_col - item.1) == 1)
            .count();
        let touches_right = hs
            .iter()
            .filter(|item| point_row == item.0 && point_col < item.1 && (item.1 - point_col) == 1)
            .count();

        let touches_up = hs
            .iter()
            .filter(|item| point_row > item.0 && point_col == item.1 && (point_row - item.0) == 1)
            .count();

        let touches_down = hs
            .iter()
            .filter(|item| point_row < item.0 && point_col == item.1 && (item.0 - point_row) == 1)
            .count();

        return (touches_left + touches_right + touches_down + touches_up) as i32;
    }

    fn add_point(&mut self, point_row: usize, point_col: usize) {
        assert!(self.touches_point(point_row, point_col));
        // Area easy - just add one.
        self.area += 1;

        // Perimeter more difficult: dependent on how much touching.
        let touches_left = self
            .items
            .iter()
            .filter(|item| point_row == item.0 && point_col > item.1 && (point_col - item.1) == 1)
            .count();

        assert!(touches_left <= 1);

        let touches_right = self
            .items
            .iter()
            .filter(|item| point_row == item.0 && point_col < item.1 && (item.1 - point_col) == 1)
            .count();
        assert!(touches_right <= 1);

        let touches_up = self
            .items
            .iter()
            .filter(|item| point_row > item.0 && point_col == item.1 && (point_row - item.0) == 1)
            .count();
        assert!(touches_up <= 1);

        let touches_down = self
            .items
            .iter()
            .filter(|item| point_row < item.0 && point_col == item.1 && (item.0 - point_row) == 1)
            .count();
        assert!(touches_down <= 1);

        let number_touching = touches_up + touches_down + touches_left + touches_right;

        match number_touching {
            0 => panic!("How are you touching 0? Touches point was true?"),
            1 => {
                self.perimeter += 2;
            } //If touching once, add 3 sides, but remove 1 from the touching side.
            2 => {
                self.perimeter += 0;
            } // If touching 2, add 2 sides, but remove 2 from touching side.
            3 => {
                self.perimeter -= 2;
            } // If touching 3, then add 1 side, but remove 3 from touching side.
            4 => {
                self.perimeter -= 4; // Should never happen, but adding anyways
            }
            _ => {
                panic!("How are you touching more than 4 sides?")
            }
        }

        self.items.insert((point_row, point_col));
    }

    fn calculate_cost(&self) -> i32 {
        self.area * self.perimeter
    }

    fn merge_with_other(&mut self, other: Self) {
        if self.charname != other.charname {
            panic!("Attempting to merge two regions without same name.");
        }
        let ensured_touching = self.items.iter().any(|it| other.touches_point(it.0, it.1));

        if !ensured_touching {
            panic!("Attempting to make two non-touching regions into one.");
        }

        println!(
            "{} -> Merging {:?} with\n\t{:?}",
            self.charname, self.items, other
        );
        self.items.extend(other.items);
        self.area = self.items.len() as i32;
        self.perimeter = self.items.iter().fold(0, |acc, it| {
            acc + 4 - Region::count_touches(self.items.clone(), it.0, it.1)
        });
    }

    fn calculate_sides_cost(&self) -> i32 {
        // TODO: finish here
        return 0;
    }
}

fn main() {
    let fileread = fs::read_to_string("inputs/day12.txt").expect("Unable to read file as string");

    let charmap = fileread
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut region_lookup: HashMap<char, Vec<Region>> = HashMap::new();

    // TODO: check that insert actually works.
    let mut test_reg = Region::new(2, 2, 'A');
    assert!(test_reg.area == 1);
    assert!(test_reg.perimeter == 4);
    assert!(test_reg.touches_point(1, 2));
    assert!(test_reg.touches_point(3, 2));
    assert!(test_reg.touches_point(2, 1));
    assert!(test_reg.touches_point(2, 3));
    assert!(!test_reg.touches_point(1, 1));

    test_reg.add_point(2, 3);
    assert!(test_reg.area == 2);
    assert!(test_reg.perimeter == 6);

    test_reg.add_point(3, 2);
    assert!(test_reg.area == 3);
    assert!(test_reg.perimeter == 8);

    // TODO: go through loop?
    for (row, line) in charmap.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            let mut regions = match region_lookup.get(ch) {
                Some(x) => (*x).clone(),
                _ => Vec::new(),
            };

            let num_touching_regions = regions
                .iter()
                .filter(|reg| reg.touches_point(row, col))
                .count();

            if num_touching_regions == 0 {
                regions.push(Region::new(row, col, *ch));
            } else if num_touching_regions == 1 {
                regions.iter_mut().for_each(|reg| {
                    if reg.touches_point(row, col) {
                        reg.add_point(row, col);
                    }
                });
            } else {
                let mut new_region = Region::new(row, col, *ch);

                let touches = regions.iter().filter(|reg| reg.touches_point(row, col));

                touches.for_each(|t| {
                    new_region.merge_with_other((*t).clone());
                });

                let mut not_touches: Vec<Region> = regions
                    .iter()
                    .filter(|&reg| !reg.touches_point(row, col))
                    .map(|reg| (*reg).clone())
                    .collect::<Vec<Region>>();

                not_touches.push(new_region);
                region_lookup.insert(*ch, not_touches);
                continue;
            }

            // Hope that never case of region combining together due to insert
            //
            region_lookup.insert(*ch, regions);
        }
    }

    region_lookup.iter().for_each(|(k, v)| {
        println!("Regions for {}", k);
        v.iter().for_each(|reg| {
            println!(
                "\t{}x{} = {}; {:?}",
                reg.area,
                reg.perimeter,
                reg.calculate_cost(),
                reg.items
            );
        });
    });

    println!(
        "Total cost: {}",
        region_lookup.values().fold(0, |acc, vec| acc
            + vec
                .iter()
                .map(|item| item.calculate_cost())
                .fold(0, |a, v| a + v))
    );
}
