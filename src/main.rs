// use crate::day5::{Crates, Stack};

use day5::stacks;


mod day5;

fn main() {
    let mut my_stack_instance: stacks::Stack<i32> = stacks::Stack::new();
    my_stack_instance.push(5);
    my_stack_instance.push(4);
    my_stack_instance.push(3);
    println!("Normal print of our data: {my_stack_instance}");

    let resp = my_stack_instance.pop();
    println!("Stack after popping top-value: {my_stack_instance}");
    println!("Response of pop: {:#?}", resp);
}
