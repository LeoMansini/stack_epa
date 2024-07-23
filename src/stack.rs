
#[derive(Debug)]
pub(crate) struct Stack<T> {
    content: Vec<T>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn new(capacity: usize) -> Self 
        where
            T: Copy + Default, 
    {
        Self {
            content: vec![T::default(); capacity],
            size: 0
        }
    }

    pub fn push(&mut self, item: T) -> () {
        self.content[self.size] = item;
        self.size = self.size + 1;
    }

    pub fn pop(&mut self) -> T 
        where
            T: Copy + Default, 
    {
        let result = self.content[self.size];
        self.content[self.size] = T::default();
        self.size = self.size - 1;
        result
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.content.len()
    }

    pub fn req_push(&self) -> bool {
        !self.is_full()
    }

    pub fn req_pop(&self) -> bool {
        !self.is_empty()
    }
}

