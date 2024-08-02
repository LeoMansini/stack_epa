use crate::stack::Stack;

pub fn non_deterministic_stack() -> Stack<usize> {
    let capacity: usize = kani::any();
    kani::assume(capacity <= 4);
    let size: usize = kani::any();
    kani::assume(size <= capacity);

    let mut stack = Stack::new(capacity);

    if stack.size() < size {
        stack.push(1);
    }
    if stack.size() < size {
        stack.push(1);
    }
    if stack.size() < size {
        stack.push(1);
    }
    if stack.size() < size {
        stack.push(1);
    }

    stack

}


#[kani::proof]
pub fn puedo_ir_push_a_pushpop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && !s.req_pop());
    s.push(1);
    assert!(s.req_push() && s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_push_a_push() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && !s.req_pop());
    s.push(1);
    assert!(s.req_push() && !s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_push_a_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && !s.req_pop());
    s.push(1);
    assert!(!s.req_push() && s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_pop_a_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(!s.req_push() && s.req_pop());
    s.pop();
    assert!(!s.req_push() && s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_pop_a_push() {
    let mut s = non_deterministic_stack();
    kani::assume(!s.req_push() && s.req_pop());
    s.pop();
    assert!(s.req_push() && !s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_pop_a_pushpop() {
    let mut s = non_deterministic_stack();
    kani::assume(!s.req_push() && s.req_pop());
    s.pop();
    assert!(s.req_push() && s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_push_con_push() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.push(1);
    assert!(s.req_push() && !s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_pop_con_push() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.push(1);
    assert!(!s.req_push() && s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_pushpop_con_push() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.push(1);
    assert!(s.req_push() && s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_push_con_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.pop();
    assert!(s.req_push() && !s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_pop_con_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.pop();
    assert!(!s.req_push() && s.req_pop())
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_pushpop_con_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.pop();
    assert!(s.req_push() && s.req_pop())
}