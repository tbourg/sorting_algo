mod lib;
use lib::*;

use std::time::{Duration, SystemTime};

fn main() {
    let mut input = vec![[4, 1], [2, 3], [1, 2], [5, 3], [4, 4]];
    let now = SystemTime::now();
    for _ in 0..1000000 {
        let mut to_sort = input.clone();
        sort(&mut to_sort);
    }
    match now.elapsed() {
        Ok(elapsed) => {
            println!("CSorting: {}", elapsed.as_nanos());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    let now = SystemTime::now();
    for _ in 0..1000000 {
        let mut to_sort = input.clone();
        to_sort.sort_by(|p1, p2| {
            if p1[0] < p2[0] {
                std::cmp::Ordering::Less
            } else if p1[0] == p2[0] {
                if p1[1] < p2[1] {
                    std::cmp::Ordering::Less
                } else if p1[1] == p2[1] {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            } else {
                std::cmp::Ordering::Greater
            }
        });
    }
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("Primitive Sorting: {}", elapsed.as_nanos());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    // println!("{:?} {:?}", input, input2);
}
