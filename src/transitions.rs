use crate::stack::Stack;

pub fn puedo_ir_push_a_pushpop(s: &mut Stack<usize>) {
    if s.req_push() && !s.req_pop() {
        s.push(1);
        assert!(s.req_push() && s.req_pop())
    }
}

pub fn puedo_ir_push_a_push(s: &mut Stack<usize>) {
    if s.req_push() && !s.req_pop() {
        s.push(1);
        assert!(s.req_push() && !s.req_pop())
    }
}

pub fn puedo_ir_push_a_pop(s: &mut Stack<usize>) {
    if s.req_push() && !s.req_pop() {
        s.push(1);
        assert!(!s.req_push() && s.req_pop())
    }
}

pub fn puedo_ir_pop_a_pop(s: &mut Stack<usize>) {
    if !s.req_push() && s.req_pop() {
        s.pop();
        assert!(!s.req_push() && s.req_pop())
    }
}

pub fn puedo_ir_pop_a_push(s: &mut Stack<usize>) {
    if !s.req_push() && s.req_pop() {
        s.pop();
        assert!(s.req_push() && !s.req_pop())
    }
}

pub fn puedo_ir_pop_a_pushpop(s: &mut Stack<usize>) {
    if !s.req_push() && s.req_pop() {
        s.pop();
        assert!(s.req_push() && s.req_pop())
    }
}

pub fn puedo_ir_pushpop_a_push_con_push(s: &mut Stack<usize>) {
    if s.req_push() && s.req_pop() {
        s.push(1);
        assert!(s.req_push() && !s.req_pop())
    }
}

pub fn puedo_ir_pushpop_a_pop_con_push(s: &mut Stack<usize>) {
    if s.req_push() && s.req_pop() {
        s.push(1);
        assert!(!s.req_push() && s.req_pop())
    }
}

pub fn puedo_ir_pushpop_a_pushpop_con_push(s: &mut Stack<usize>) {
    if s.req_push() && s.req_pop() {
        s.push(1);
        assert!(s.req_push() && s.req_pop())
    }
}

pub fn puedo_ir_pushpop_a_push_con_pop(s: &mut Stack<usize>) {
    if s.req_push() && s.req_pop() {
        s.pop();
        assert!(s.req_push() && !s.req_pop())
    }
}

pub fn puedo_ir_pushpop_a_pop_con_pop(s: &mut Stack<usize>) {
    if s.req_push() && s.req_pop() {
        s.pop();
        assert!(!s.req_push() && s.req_pop())
    }
}

pub fn puedo_ir_pushpop_a_pushpop_con_pop(s: &mut Stack<usize>) {
    if s.req_push() && s.req_pop() {
        s.pop();
        assert!(s.req_push() && s.req_pop())
    }
}