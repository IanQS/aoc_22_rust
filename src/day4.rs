use std::{fmt, fs};
use Vec;

struct Range {
    start: i32,
    end: i32,
}


impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

fn read_file(f_name: &str) -> Vec<String> {
    let read_str: Vec<String> = fs::read_to_string(f_name)
        .expect("Error reading file")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())

        .collect();

    let replaced_one: Vec<String> = read_str.iter()
        .map(|x| x.replace("-", " "))
        .map(|x| x.replace(",", " "))
        .collect();

    return replaced_one;
}

pub fn problem1(f_name: &str) -> usize {
    let split_values = read_file(f_name);

    let filtered_results: Vec<bool> = split_values.iter()
        // Generate the two ranges Range1(start1, end1) and Range2(start2, end2)
        .map(|line| {
            let split_ln: Vec<i32> = line.split(" ").map(|v| v.parse::<i32>().unwrap()).collect();


            let (s1, e1, s2, e2) = match split_ln[..] {
                [s1, e1, s2, e2, ..] => (s1, e1, s2, e2),
                _ => unreachable!(),
            };

            let range1 = Range {
                start: s1,
                end: e1,
            };
            let range2 = Range {
                start: s2,
                end: e2,
            };
            return (range1, range2);
        })
        .map(|ranges| {
            let (r1, r2) :(Range, Range) = (ranges.0, ranges.1);
            return (r1.start <= r2.start && r1.end >= r2.end) || (r2.start <= r1.start && r2.end >= r1.end);
        })
        .filter(|x| *x)
        .collect()
        ;

    return filtered_results.len();
}