use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Stack<T> {
    data_vec: Vec<T>,
}

/// Discussion on how you create "classes"/ implementations in Rust
///https://www.reddit.com/r/rust/comments/7llmu1/why_do_you_need_to_declare_generic_twice_in_impl/
/// https://stackoverflow.com/questions/45473626/why-does-rust-require-generic-type-declarations-after-the-impl-keyword/45473717#45473717
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { data_vec: vec![]}
    }

    pub fn push(&mut self, val: T) -> () {
        self.data_vec.push(val);
    }

    pub fn len(&self) -> usize{
        return self.data_vec.len();
    }

    pub fn pop(&mut self) -> Result<T, String> {
        if !self.data_vec.is_empty() {
            match self.data_vec.pop() {
                Some(val) => Ok(val),
                None => Err("Error while popping array".to_string())
            }
        } else {
            return Err("Tried to pop an empty stack".parse().unwrap());
        }
    }
}

impl<T> fmt::Display for Stack<T> where
    T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stack: [")?;
        for el in self.data_vec.iter().rev() {
            write!(f, "{el}, ")?;
        }
        write!(f, "]")
    }
}


#[cfg(test)]
mod tests {
    use crate::day5::stacks::Stack;

    #[test]
    fn stack_creation() -> Result<(), String>{
        let some_stack: Stack<i32> = Stack::new();
        assert_eq!(some_stack.len(), 0);
        Ok(())
    }

    #[test]
    fn stack_append() -> Result<(), String>{
        let to_push = 5;
        let mut some_stack: Stack<i32> = Stack::new();
        some_stack.push(to_push);
        let stack_popped = some_stack.pop();
        match stack_popped{
            Ok(val) => {
                assert_eq!(val, to_push);
            },
            Err(_) => {return Err("Failed on first pop when popping non-empty stack".to_string());}
        }

        let stack_popped = some_stack.pop();
        match stack_popped {
            Ok(_) => {return Err("Second pop returned non-err ".to_string())},
            Err(_) => {return Ok(());},
        }
    }
}
