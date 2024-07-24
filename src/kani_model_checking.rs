use crate::transitions::puedo_ir_pushpop_a_pop_con_pop;
use crate::stack::Stack;

#[cfg(kani)]
#[kani::proof]
fn test_stack_transitions() {
    transition_exists(
        &|stack| stack.req_pop() && stack.req_push(),
        &|stack| stack.req_pop() && !stack.req_push(),
        &mut |stack| { stack.push(1); }
    )
}

#[cfg(kani)]
pub fn non_deterministic_stack() -> Stack<usize> {
    let s = Stack {
        content: kani::vec::exact_vec::<usize, 2>(),
        size: kani::any()
    };
    kani::assume(Stack::size_is_valid(&s));
    s
}

#[cfg(kani)]
pub fn transition_exists(
    precondition_a: &dyn Fn(&Stack<usize>) -> bool,
    precondition_b: &dyn Fn(&Stack<usize>) -> bool,
    method: &mut dyn FnMut(&mut Stack<usize>),
) {
    let mut s = non_deterministic_stack();
    kani::assume(precondition_a(&s));
    method(&mut s);
    assert!(precondition_b(&s));
}