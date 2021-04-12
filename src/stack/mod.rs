#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

// Para implementación con vector de tamaño fijo: ArrayVec
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.stack.last()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.stack.clear()
    }
}

#[cfg(test)]

mod test {
    #[test]
    fn test_new_stack() {
        let mut stack = super::Stack::<u32>::new();
        assert!(stack.empty());
        stack.push(3);
        assert_eq!(Some(&3u32), stack.top());
    }

    #[test]
    fn test_push() {
        let mut stack = super::Stack::<u32>::new();
        stack.push(3);
        assert_eq!(1, stack.size());
    }

    #[test]
    fn test_pop_top() {
        let mut stack = super::Stack::<u32>::new();
        stack.push(1);
        assert_eq!(Some(&1u32), stack.top());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

}