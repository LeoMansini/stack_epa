#[derive(Debug)]
pub struct Stack<T> {
    content: Vec<T>,
    capacity: usize,
    size: usize,
}

impl<T> Stack<T> {
    // Create a new, empty stack
    pub fn new(capacity: usize) -> Self {
        Stack { 
            content: Vec::new(),
            capacity: capacity,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.is_full() {
            panic!("Stack is full");
        }
        self.size = self.size + 1;
        self.content.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            panic!("Stack is empty")
        }
        self.size = self.size - 1;
        self.content.pop()
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

    pub fn size(&self) -> usize {
        self.size
    }
}