use std::fmt;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

use crate::day5::stacks::Stack;

#[derive(Debug)]
// Define a public crate, but the constructor is not public.
pub struct Crates<T> {
    crates: HashMap<String, Stack<T>>,
    num_entries: i32,
}

impl<T> Crates<T> {
    pub fn new(n: i32) -> Self {
        let mut crate_obj = Crates {
            crates: HashMap::new(),
            num_entries: 0,
        };

        for i in 1..n + 1 {
            let crate_name = crate_obj.get_as_crate_idx(i);
            crate_obj.crates.insert(crate_name, Stack::new());
        }
        return crate_obj;
    }

    fn get_as_crate_idx(&self, i: i32) -> String {
        let idx_string: String = i.to_string().to_owned();
        let mut crate_name_prefix: String = "Crate".to_string();
        crate_name_prefix.push_str(&idx_string);
        return crate_name_prefix;
    }

    pub fn push(&mut self, to_push: T, stack_num: i32) -> Result<bool, String> {
        let crate_name = self.get_as_crate_idx(stack_num);
        match self.crates.get_mut(crate_name.as_str()) {
            Some(v) => {
                v.push(to_push);
                return Ok(true);
            }
            _ => {
                return Err("Specified crate does not exist".to_string());
            }
        }
    }

    pub fn pop(&mut self, stack_num: i32) -> Result<T, String>{
        let this_stack = self.crates.get_mut(
            self.get_as_crate_idx(stack_num).as_str()
        );
        match this_stack {
            Some(stack) => {return stack.pop()},
            None => {return Err("Popped an empty stack".to_string());}
        }
    }
}

impl<T> fmt::Display for Crates<T>
    where
        T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for i in 1..self.num_entries {
            let curr_stack_name = self.get_as_crate_idx(i);
            let curr_stack = self.crates.get(curr_stack_name.as_str());
            match curr_stack{
                Some(v) => {
                    write!(f, "\tStack {}: ", curr_stack_name)?;
                    write!(f, "{}", v)?;
                    writeln!(f, "]")?;
                }
                None => (write!(f, "Somehow tried to access a stack we do not have..."))?,
            }
        }
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests{
    use crate::day5::crates_sol::Crates;
    use crate::day5::stacks::Stack;

    // Used in the case of insert and pop
    fn crate_create_insert_util(num_crates: i32, num_insert: i32) -> Crates<i32>{
        let mut some_crate: Crates<i32> = Crates::new(num_crates);

        // test insertion
        for crate_idx in 1..num_crates+1{
            for j in 0..num_insert{
                let to_insert = j + (crate_idx * 10);
                let _ = some_crate.push(to_insert, crate_idx);
            }
        }
        return some_crate;
    }

    #[test]
    fn crate_creation() -> Result<(), String>{
        let num_crates = 4;
        let some_crate: Crates<i32> = Crates::new(num_crates);
        assert_eq!(some_crate.crates.len(), num_crates as usize);
        Ok(())
    }

    #[test]
    fn as_crate_idx() -> Result<(), String>{
        let num_crates = 4;
        let some_crate: Crates<i32> = Crates::new(num_crates);
        let crate_name1 = some_crate.get_as_crate_idx(num_crates);
        assert_eq!(crate_name1, "Crate4".to_string());

        let crate_name1 = some_crate.get_as_crate_idx(-1);
        assert_eq!(crate_name1, "Crate-1".to_string());
        Ok(())
    }

    #[test]
    fn simple_crate_append_test() -> Result<(), String>{
        let num_crates = 4;
        let num_insert = 5;
        let mut this_crate = crate_create_insert_util(
            num_crates,
            num_insert
        );

        for crate_idx in 0..num_crates{
            let this_stack = this_crate
                .pop(crate_idx);
            let mut count = 0;
            match this_stack {
                Ok(v) => {
                    count += 1;
                    assert_eq!(v, ((crate_idx + 1) * 10) - num_insert - count);
                },
                Err(_) => {break;}
            }
        }
        Ok(())
    }
}