use crate::stack::Stack;
use crate::transitions::*;

#[cfg(kani)]
#[kani::proof]
fn test_stack_transitions() {
    puedo_ir_push_a_pushpop();
    puedo_ir_push_a_push();
    puedo_ir_push_a_pop();

    puedo_ir_pop_a_pushpop();
    puedo_ir_pop_a_pop();
    puedo_ir_pop_a_push();

    puedo_ir_pushpop_a_pushpop_con_push();
    puedo_ir_pushpop_a_pop_con_push();
    puedo_ir_pushpop_a_push_con_push();

    puedo_ir_pushpop_a_pushpop_con_pop();
    puedo_ir_pushpop_a_pop_con_pop();
    puedo_ir_pushpop_a_push_con_pop();

}


#[cfg(kani)]
pub fn non_deterministic_stack() -> Stack<usize> {
    let s = Stack {
        content: kani::vec::exact_vec::<usize, 3>(), // Fixes capacity to two without loss of generality
        size: kani::any()
    };
    kani::assume(Stack::size_is_valid(&s));
    s
}