use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

/// Discussion on how you create "classes"/ implementations in Rust
///
/// https://www.reddit.com/r/rust/comments/7llmu1/why_do_you_need_to_declare_generic_twice_in_impl/
#[derive(Debug)]
pub struct Stack<T>{
    data_vec: Vec<T>,
}

impl<T> Stack<T>{
    pub fn new() -> Self{
        Stack { data_vec: vec![]}
    }

    pub fn push(&mut self, val: T) -> (){
        self.data_vec.push(val);
    }

    pub fn pop(&mut self) -> Result<T, String>{
        if self.data_vec.is_empty(){
            match self.data_vec.pop(){
                Some(val) => Ok(val),
                None => Err("Error while popping array".to_string())
            }
        } else{
            return Err("Tried to pop an empty stack".parse().unwrap());
        }
    }
}

#[derive(Debug)]
// Define a public crate, but the constructor is not public.
pub struct Crates<T>{
    crates: HashMap<String, Stack<T>>,
    num_entries: i32
}

impl<T> Crates<T> {
    pub fn new(n: i32) -> Self{
        let mut crate_obj = Crates {
            crates: HashMap::new(),
            num_entries: 0
        };

        for i in 1..n+1{
            let crate_name = crate_obj.get_as_crate_idx(i);
            crate_obj.crates.insert(crate_name, Stack::new());
        }
        return crate_obj;
    }

    fn get_as_crate_idx(&self, i: i32) -> String{
        let idx_string: String = i.to_string().to_owned();
        let mut crate_name_prefix: String = "Crate".to_string();
        crate_name_prefix.push_str(&idx_string);
        return crate_name_prefix;
    }
}

impl fmt::Display for Crates<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 1..self.num_entries{
            let curr_stack_name = self.get_as_crate_idx(i);
            let curr_stack = self.crates.get(curr_stack_name.as_str());

            for el in curr_stack
            print!("Stack {}: ", i);

            let curr_s

        }
        for entry in self.stack1{
            print!("{}, ", entry);
        };
        println!("");



    }
}