use crate::transitions::puedo_ir_pushpop_a_pop_con_pop;
use crate::stack::Stack;

#[cfg(kani)]
#[kani::proof]
fn test_stack_transitions() {
    let pop_push_state: Box<dyn Fn(&Stack<usize>) -> bool> = Box::new(|stack: &Stack<usize>| stack.req_pop() && stack.req_push());
    let pop_state: Box<dyn Fn(&Stack<usize>) -> bool> = Box::new(|stack: &Stack<usize>| stack.req_pop() && !stack.req_push());
    let push_state: Box<dyn Fn(&Stack<usize>) -> bool> = Box::new(|stack: &Stack<usize>| !stack.req_pop() && stack.req_push());

    // Define closures for the methods
    let push_method: Box<dyn FnMut(&mut Stack<usize>)> = Box::new(|stack: &mut Stack<usize>| { stack.push(kani::any()); });
    let pop_method: Box<dyn FnMut(&mut Stack<usize>)> = Box::new(|stack: &mut Stack<usize>| { stack.pop(); });

    // Store the closures in vectors
    let states = [pop_push_state, pop_state, push_state];
    let mut methods = [push_method, pop_method];

    // Iterate over all combinations of states and methods
    for start_state in &states {
        for end_state in &states {
            for method in &mut methods {
                transition_exists(
                    start_state.as_ref(),
                    end_state.as_ref(),
                    method.as_mut(),
                );
            }
        }   
    }
}


#[cfg(kani)]
pub fn non_deterministic_stack() -> Stack<usize> {
    let s = Stack {
        content: kani::vec::exact_vec::<usize, 2>(), // Fixes capacity to two without loss of generality
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