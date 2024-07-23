use crate::transitions::puedo_ir_pushpop_a_pop_con_pop;
use crate::stack::Stack;

#[cfg(kani)]
#[kani::proof]
fn test_stack_operations() {
    let s = Stack {
        content: kani::any(),
        size: kani::any()
    };
    puedo_ir_pushpop_a_pop_con_pop(&mut s);
}