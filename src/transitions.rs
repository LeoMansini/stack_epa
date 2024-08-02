use crate::stack::Stack;

pub fn non_deterministic_stack() -> Stack<usize> {
    let bound: usize = kani::any();
    kani::assume(bound <= 4);
    let size: usize = kani::any();
    kani::assume(size <= bound);

    let mut stack = Stack::new(bound);

    for _ in 0..size {
        let item: usize = 1;
        stack.push(item);
    }

    stack

}


#[kani::proof]
#[kani::unwind(16)]
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