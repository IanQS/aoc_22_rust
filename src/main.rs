// mod day1;
// mod day2;
mod day3;

fn main() {
    // let result = day3::part1_calculate_priorities("./data_folder/day_3.txt");
    // println!("Day 3: Part 1: {}", result);

    let result = day3::part2_calculate_priorities_triples(
        "./data_folder/day_3.txt"
    );
    println!("Final result: {}", result);
}
