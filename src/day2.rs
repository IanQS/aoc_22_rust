//!
//! Day 2 documentation. Commented out section is for problem 1
//! Note: in problem 2 we use the convention
//!     {X + Y} where X is the score for choosing our move, and Y is the score for the result of the game

use std::fs;
use std::str::Split;

// Read the file and return each round as a vector of strings
pub fn read_file(f_name: &str) -> Vec<String> {
    let contents: Vec<String> = fs::read_to_string(f_name)
        // Custom-msg version of unwrap
        .expect("Could not read input file")  // Result -> &str (due to unpacking)
        .split("\n") // Iter<&str>
        .map(|x| x.to_string())  // Iter<String>
        .filter(|x| !x.is_empty()) // remove empty strings (last line messed us up)
        .collect();
    return contents;
}
//
// fn initial_score(my_move: &str) -> i32 {
//     return match my_move {
//         "X" => { 1 }
//         "Y" => { 2 }
//         "Z" => { 3 }
//         _ => { 0 }
//     }
// }

// // rock - problem 1
// fn handle_move_a(my_move: &str) -> i32 {
//     let total_score = initial_score(my_move);
//     return match my_move {
//         "X" => { total_score + 3 } // a draw
//         "Y" => { total_score + 6 } // a win
//         "Z" => { total_score + 0 } // a loss
//         _ => { total_score + 0 }
//     }
// }
//
// // Paper - problem 1
// fn handle_move_b(my_move: &str) -> i32 {
//     let total_score = initial_score(my_move);
//     return match my_move {
//         "X" => { total_score + 0 }
//         "Y" => { total_score + 3 }
//         "Z" => { total_score + 6 }
//         _ => { total_score + 0 }
//     }
// }
// //scissors - problem 1
// fn handle_move_c(my_move: &str) -> i32 {
//     let total_score = initial_score(my_move);
//     return match my_move {
//         "X" => { total_score + 6 }
//         "Y" => { total_score + 0 }
//         "Z" => { total_score + 3 }
//         _ => { total_score + 0 }
//     }
// }

fn want_lose(their_move: &str) -> i32{
    return match their_move {
        "A" => 3,
        "B" => 1,
        "C" => 2,
        _ => 0
    }
}

fn want_draw(their_move: &str) -> i32{
    return match their_move {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0
    }
}
fn want_win(their_move: &str) -> i32{
    return match their_move {
        "A" => 2,
        "B" => 3,
        "C" => 1,
        _ => 0
    }
}
// The function that we map over the iters - problem 1
fn mapper_function(plays: Split<&str>) -> i32 {
    let split_plays: Vec<&str> = plays.collect();
    let (their_move, desired_result): (&str, &str) = (split_plays[0], split_plays[1]);
    return match desired_result {
        "X" => {want_lose(their_move) + 0},
        "Y" => {want_draw(their_move) + 3},
        "Z" => {want_win(their_move) + 6},
        _ => {0}
    };
}


pub fn calculate_score(rounds_as_string: Vec<String>) -> i32 {
    return rounds_as_string
        .iter()// first generate an iterator from this Vec. We do so to map over it
        .map(|x| mapper_function(x.split(" "))) // map our mapper function over it
        .sum();  // collect the results back into a vector.
}
