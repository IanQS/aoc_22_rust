/// Implement this using whatever you've learned from `day5.md`. You can just refer to `crates_sol.rs` for the solutions
///
use std::fmt;
use std::collections::HashMap;
use std::fmt::Debug;
use crate::day5::stacks::Stack;

#[derive(Debug)]
// Define a public crate, but the constructor is not public.
pub struct Crates<T> {
    crates: HashMap<String, Stack<T>>,
    num_entries: i32,
}

impl<T> Crates<T> {
    pub fn new(n: i32) -> Self {
        todo!();
    }


    pub fn push(&mut self, to_push: T, crate_num: i32) -> Result<bool, String> {
        todo!();
    }
}

impl<T> fmt::Display for Crates<T>
    where
        T: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}
