mod stack;
use stack::Stack;

fn main() {
    println!("Hello, world!");
    let mut s: Stack<usize> = Stack::new(2usize);
    puedo_ir_push_a_pushpop(&mut s);
}

fn puedo_ir_push_a_pushpop(s: &mut Stack<usize>) {
    if s.req_push() && !s.req_pop() {
        s.push(1);
        assert!(s.req_push() && s.req_pop())
    }
}

fn puedo_ir_push_a_push(s: &mut Stack<usize>) {
    if s.req_push() && !s.req_pop() {
        s.push(1);
        assert!(s.req_push() && !s.req_pop())
    }
}

fn puedo_ir_push_a_pop(s: &mut Stack<usize>) {
    if s.req_push() && !s.req_pop() {
        s.push(1);
        assert!(!s.req_push() && s.req_pop())
    }
}

fn puedo_ir_pop_a_pop(s: &mut Stack<usize>) {
    if !s.req_push() && s.req_pop() {
        s.push(1);
        assert!(!s.req_push() && s.req_pop())
    }
}

fn puedo_ir_pop_a_push(s: &mut Stack<usize>) {
    if !s.req_push() && s.req_pop() {
        s.push(1);
        assert!(s.req_push() && !s.req_pop())
    }
}

fn puedo_ir_pop_a_pushpop(s: &mut Stack<usize>) {
    if !s.req_push() && s.req_pop() {
        s.push(1);
        assert!(s.req_push() && s.req_pop())
    }
}

fn puedo_ir_pushpop_a_push(s: &mut Stack<usize>) {
    if s.req_push() && s.req_pop() {
        s.push(1);
        assert!(s.req_push() && !s.req_pop())
    }
}

fn puedo_ir_pushpop_a_pop(s: &mut Stack<usize>) {
    if s.req_push() && s.req_pop() {
        s.push(1);
        assert!(!s.req_push() && s.req_pop())
    }
}

fn puedo_ir_pushpop_a_pushpop(s: &mut Stack<usize>) {
    if s.req_push() && s.req_pop() {
        s.push(1);
        assert!(s.req_push() && s.req_pop())
    }
}