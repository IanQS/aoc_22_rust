use std::fs;
use std::collections::HashMap;
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