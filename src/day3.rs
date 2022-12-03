//! Problem 1
//!     **Note**: I used a dict when I should have used a set
//!     Iterate through the strings by first splitting them in 1/2
//!     Find the intersection and get the idx before summing
//! Problem 2:
//!     **Note**: same as earlier but I'm now doing things a little different
//!

use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::day3::constants::ASCII;

mod constants;


fn read_file(f_name: &str) -> Vec<String> {
    return fs::read_to_string(f_name)
        .expect("")
        .split("\n")
        .map(
            |x| x.to_string()
        )
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();
}
#[allow(dead_code)]
fn split_str_into_map(my_str: &str, h_map: &mut HashMap<char, i32>) {
    my_str.chars().fold(
        h_map,
        |h_map, _char| {
            *h_map.entry(_char).or_insert(0) += 1;
            return h_map;
        },
    );
    return;
}
#[allow(dead_code)]
pub fn part1_calculate_priorities(f_name: &str) -> usize {
    let contents = read_file(f_name);

    // First we split in half by length
    return contents.iter()
        // First we split s.t we now have an Iter<&str, &str>
        .map(
            |x| x.split_at(x.len() / 2)
        )
        // Now we do a map to accumulate a vector of tuples of dictionaries
        .map(
            |str_tup: (&str, &str)|
                {
                    let mut h_map_1 = HashMap::<char, i32>::new();
                    let mut h_map_2 = HashMap::<char, i32>::new();
                    split_str_into_map(str_tup.0, &mut h_map_1);
                    split_str_into_map(str_tup.1, &mut h_map_2);
                    return (h_map_1, h_map_2);
                }
        )
        // Now we recognize that each vector entry is independent. So, we do another
        // map and this time we intersect the keys of the two dictionaries
        .map(
            |dict_tup| {
                let mut sum_ret = 1;  // to account for 0 index
                for kv in dict_tup.0.iter() {
                    if dict_tup.1.contains_key(kv.0) {
                        sum_ret += ASCII.iter().position(|curr_char| curr_char == kv.0).unwrap()
                    }
                }
                return sum_ret;
            }
        ).sum();
}


pub fn part2_calculate_priorities_triples(f_name: &str) -> usize{
    // First read in all the lines then split by 3
    let contents = read_file(f_name);

    // for each entry, we get the set associated with it
    let vec_set: Vec<HashSet<char>> = contents.iter()
        .map(|x| {
            x.chars().into_iter().collect()
        })
        .collect();

    let mut total_priorities = 0;
    for i in (0..vec_set.len()).step_by(3){
        let fst = &vec_set[i];
        let sec = &vec_set[i+1];
        let third = &vec_set[i+2];
        let intersect = fst.
            intersection(sec).into_iter().filter(
            |x| third.contains(x)
        ).into_iter();

        for (i, chr) in intersect.enumerate(){
            total_priorities += ASCII.iter().position(|ascii_chars| ascii_chars == chr).unwrap();
            assert!(i == 0, "More than 1 element in the intersection");
        }
        total_priorities += 1;
    }
    return total_priorities;
}