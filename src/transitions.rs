use crate::stack::Stack;

#[cfg(kani)]
pub fn non_deterministic_stack() -> Stack<usize> {
    let capacity: usize = kani::any();
    kani::assume(capacity <= 4);

    let num_elements: usize = kani::any();
    kani::assume(num_elements <= capacity);

    let mut content = kani::vec::any_vec::<usize, 4>();

    kani::assume(content.len() <= capacity);

    let s = Stack::with_content(capacity, content);
    s
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_push_a_pushpop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && !s.req_pop());
    s.push(1);
    assert!(s.req_push() && s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_push_a_push() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && !s.req_pop());
    s.push(1);
    assert!(s.req_push() && !s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_push_a_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && !s.req_pop());
    s.push(1);
    assert!(!s.req_push() && s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_pop_a_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(!s.req_push() && s.req_pop());
    s.pop();
    assert!(!s.req_push() && s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_pop_a_push() {
    let mut s = non_deterministic_stack();
    kani::assume(!s.req_push() && s.req_pop());
    s.pop();
    assert!(s.req_push() && !s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_pop_a_pushpop() {
    let mut s = non_deterministic_stack();
    kani::assume(!s.req_push() && s.req_pop());
    s.pop();
    assert!(s.req_push() && s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_pushpop_a_push_con_push() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.push(1);
    assert!(s.req_push() && !s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_pushpop_a_pop_con_push() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.push(1);
    assert!(!s.req_push() && s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_pushpop_a_pushpop_con_push() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.push(1);
    assert!(s.req_push() && s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_pushpop_a_push_con_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.pop();
    assert!(s.req_push() && !s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_pushpop_a_pop_con_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.pop();
    assert!(!s.req_push() && s.req_pop())
}

#[cfg(kani)]
#[kani::proof]
pub fn puedo_ir_pushpop_a_pushpop_con_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(s.req_push() && s.req_pop());
    s.pop();
    assert!(s.req_push() && s.req_pop())
}