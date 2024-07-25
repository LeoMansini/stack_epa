
#[derive(Debug)]
pub(crate) struct Stack<T> {
    content: Vec<T>,
    capacity: usize,
    size: usize
}

impl<T> Stack<T> {
    pub fn new(capacity: usize) -> Self 
        where
            T: Copy, 
    {
        Self {
            content: Vec::with_capacity(capacity),
            capacity: capacity,
            size: 0
        }
    }

    pub fn with_content(capacity: usize, content: Vec<T>) -> Self
        where
            T: Copy,
    {
        let content_size = content.len();
        if content_size > capacity {
            panic!("Stack with more items than capacity.")
        }
        Self {
            content: content,
            capacity: capacity,
            size: content_size
        }
    }

    pub fn push(&mut self, item: T) -> () {
        if self.is_full() {
            panic!("Stack is already full.")
        }
        self.size = self.size + 1;
        self.content.push(item);
    }

    pub fn pop(&mut self) -> T 
        where
            T: Copy, 
    {
        self.size = self.size - 1;
        self.content.pop().unwrap()
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

