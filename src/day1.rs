//!
//! Day 1 of the challenge including problem 1 and 2.
use std::cmp::max;
use std::fs;
use std::collections::BinaryHeap;

/// Scan and break on newline
pub(crate) fn find_max_from_file(file_name: &str) -> i32{
    println!("Loading from file {}", file_name);
    let _contents =fs::read_to_string(file_name).unwrap();
    let mut running_max: i32 = 0;
    let mut _curr_sum: i32=0;
    for ln in _contents.split("\n"){
        match ln.parse::<i32>() {
            Ok(v) => {
                _curr_sum += v;
            }
            Err(_) => {
                running_max = max(running_max, _curr_sum);
                _curr_sum = 0;
            }
        };
    }
    return running_max;
}

/// Finds the top-n and sums them up
///
pub(crate) fn find_sum_top_n_from_file(file_name: &str, n: i32) -> i32{
    println!("Loading from file {}", file_name);
    let _contents =fs::read_to_string(file_name).unwrap();
    let mut heap : BinaryHeap<i32> = BinaryHeap::new();
    let mut _curr_sum: i32=0;
    for ln in _contents.split("\n"){
        match ln.parse::<i32>() {
            Ok(v) => {
                _curr_sum += v;
            }
            Err(_) => {
                heap.push(_curr_sum);
                _curr_sum = 0;
            }
        };
    }
    let mut final_result = 0;
    for i in 0..n {
        match heap.pop(){
            Some(x) => {final_result += x},
            _ => ()
        };

    }
    return final_result;
}
