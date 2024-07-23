use crate::transitions::puedo_ir_pushpop_a_pop_con_pop;
use crate::stack::Stack;

#[cfg(kani)]
#[kani::proof]
fn test_stack_operations() {
    let mut s: Stack<usize> = Stack::new(2usize);
    s.push(1);
    puedo_ir_pushpop_a_pop_con_pop(&mut s);
}