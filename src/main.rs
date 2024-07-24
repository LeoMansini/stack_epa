mod stack;
use stack::Stack;
mod kani_model_checking;

mod transitions;
use transitions::puedo_ir_push_a_pushpop;

fn main() {
    println!("Hello, world!");
    let mut s: Stack<usize> = Stack::new(2usize);
    //puedo_ir_push_a_pushpop(&mut s);
}

