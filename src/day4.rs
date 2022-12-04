use std::{fmt, fs};
use Vec;

struct Range {
    start: i32,
    end: i32,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.start, self.end)
    }
}

fn read_file(f_name: &str) -> Vec<String> {
    let v: String = fs::read_to_string(f_name)
        .expect("Error reading file");

    let b: Vec<&str> = v
        // split on ,newlines. -> {a-b, c-d,}, {range_2}, {range_3},...
        .split(",\n")
        .flat_map(
            |two_ranges| two_ranges.split(",").collect::<Vec<&str>>() // a-b then c-d
        )
        .collect()
        ;

    for el in b.iter(){
        print!("{}", el);
    }

    return vec!["A".to_string()];
}

pub fn problem1(f_name: &str) -> i32{
    let split_values = read_file(f_name);

    for vec in split_values.iter(){
        print!("Length: {}", vec.len())
    }
    //
    // let ranges = split_values.iter()
    //     .map(|line| {
    //         println!("String:? {}", line);
    //         let split_ln: Vec<&str>= line.split(" ").collect();
    //
    //         let range1 = Range{
    //             start: split_ln[0].parse::<i32>().unwrap(),
    //             end: split_ln[1].parse::<i32>().unwrap()
    //         };
    //         let range2 = Range{
    //             start: split_ln[3].parse::<i32>().unwrap(),
    //             end: split_ln[4].parse::<i32>().unwrap()
    //         };
    //         return (range1, range2);
    //     })
    //     ;
    // for el in ranges.into_iter(){
    //     println!("Elements: {} and {}", el.0, el.1)
    // }
    return 0;
}