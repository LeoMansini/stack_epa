use crate::stack::Stack;

pub fn non_deterministic_stack() -> Stack<usize> {
    let capacity: usize = kani::any();
    kani::assume(capacity <= 3); // Suficiente para posibilitar todas las trancisiones.
    let size: usize = kani::any();
    kani::assume(size <= capacity);

    let mut stack = Stack::new(capacity);

    // Unwind manual del loop
    if stack.size() < size {
        stack.push(1); // Transiciones no dependen del contenido.
    }
    if stack.size() < size {
        stack.push(1);
    }
    if stack.size() < size {
        stack.push(1);
    }

    stack

}

pub fn estoy_en_pop(s: &Stack<usize>) -> bool {
    !s.req_push() && s.req_pop()
}

pub fn estoy_en_push(s: &Stack<usize>) -> bool {
    s.req_push() && !s.req_pop()
}

pub fn estoy_en_pushpop(s: &Stack<usize>) -> bool {
    s.req_push() && s.req_pop()
}

pub fn estoy_en_nada(s: &Stack<usize>) -> bool {
    !s.req_push() && !s.req_pop()
}


#[kani::proof]
pub fn puedo_ir_push_a_pushpop() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_push(&s));
    s.push(1);
    assert!(!estoy_en_pushpop(&s))
}


#[kani::proof]
pub fn puedo_ir_push_a_push() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_push(&s));
    s.push(1);
    assert!(!estoy_en_push(&s))
}


#[kani::proof]
pub fn puedo_ir_push_a_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_push(&s));
    s.push(1);
    assert!(!estoy_en_pop(&s))
}


#[kani::proof]
pub fn puedo_ir_pop_a_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_pop(&s));
    s.pop();
    assert!(!estoy_en_pop(&s))
}


#[kani::proof]
pub fn puedo_ir_pop_a_push() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_pop(&s));
    s.pop();
    assert!(!estoy_en_push(&s))
}


#[kani::proof]
pub fn puedo_ir_pop_a_pushpop() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_pop(&s));
    s.pop();
    assert!(!estoy_en_pushpop(&s))
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_push_con_push() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_pushpop(&s));
    s.push(1);
    assert!(!estoy_en_push(&s))
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_pop_con_push() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_pushpop(&s));
    s.push(1);
    assert!(!estoy_en_pop(&s))
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_pushpop_con_push() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_pushpop(&s));
    s.push(1);
    assert!(!estoy_en_pushpop(&s))
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_push_con_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_pushpop(&s));
    s.pop();
    assert!(!estoy_en_push(&s))
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_pop_con_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_pushpop(&s));
    s.pop();
    assert!(!estoy_en_pop(&s))
}


#[kani::proof]
pub fn puedo_ir_pushpop_a_pushpop_con_pop() {
    let mut s = non_deterministic_stack();
    kani::assume(estoy_en_pushpop(&s));
    s.pop();
    assert!(!estoy_en_pushpop(&s))
}
