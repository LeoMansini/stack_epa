use crate::transitions::puedo_ir_push_a_pushpop;
use crate::stack::Stack;

#[cfg(kani)]
#[kani::proof]
fn test_stack_operations() {
    let mut s: Stack<usize> = Stack::new(2usize);
    puedo_ir_push_a_pushpop(&mut s);
}