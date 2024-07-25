
#[derive(Debug)]
pub(crate) struct Stack<T> {
    content: Vec<T>,
    capacity: usize,
}

impl<T> Stack<T> {
    pub fn new(capacity: usize) -> Self 
        where
            T: Copy, 
    {
        Self {
            content: Vec::with_capacity(capacity),
            capacity
        }
    }

    pub fn push(&mut self, item: T) -> () {
        if self.is_full() {
            panic!("Stack is already full.")
        }
        self.content.push(item);
    }

    pub fn pop(&mut self) -> T 
        where
            T: Copy, 
    {
        self.content.pop().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.content.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.capacity == self.content.len()
    }

    pub fn req_push(&self) -> bool {
        !self.is_full()
    }

    pub fn req_pop(&self) -> bool {
        !self.is_empty()
    }

    pub fn size_is_valid(&self) -> bool {
        self.size < self.content.len()
    }
}

