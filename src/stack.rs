#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    capacity: usize,
    size: usize,
}

impl<T> Stack<T> {
    // Create a new, empty stack
    pub fn new(capacity: usize) -> Self {
        Stack { 
            top: None,
            capacity: capacity,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.is_full() {
            panic!("Stack is full");
        }
        let new_node = Box::new(Node {
            value,
            next: self.top.take(),
        });
        self.top = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            panic!("Stack is empty")
        }
        self.top.take().map(|node| {
            self.top = node.next;
            node.value
        })
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.capacity == self.size
    }

    pub fn req_push(&self) -> bool {
        !self.is_full()
    }

    pub fn req_pop(&self) -> bool {
        !self.is_empty()
    }
}