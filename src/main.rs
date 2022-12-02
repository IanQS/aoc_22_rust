mod day1;
mod day2;

fn main() {
    let result: i32 = day2::find_sum_top_n_from_file(
        "./data_folder/day_1_2.txt",
        3
    );
    println!("Final result: {}", result);

}
