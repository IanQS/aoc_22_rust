use std::cmp::max;
use std::fs;

pub(crate) fn find_max_from_file(file_name: &str) -> i32{
    println!("Loading from file {}", file_name);
    let _contents =fs::read_to_string(file_name).unwrap();
    let mut running_max: i32 = 0;
    let mut _curr_sum: i32=0;
    for ln in _contents.split("\n"){
        match ln.parse::<i32>() {
            Ok(v) => {
                println!("Curr val: {}", ln);
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