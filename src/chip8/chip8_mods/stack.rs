use std::collections::VecDeque;

pub struct Stack {
    stack_queue: Vec<u16>,
    pointer: usize
}

impl Stack {
    pub fn new() -> Self {
        Self { stack_queue: Vec::new(),
                pointer: 0}
    }

    pub fn push(&mut self, val: u16) {
        self.stack_queue.push(val);
        if self.stack_queue.len() > 16 { 
            println!("STACK OVERFLOW ON CHIP8 STACK!");
        }
    }

    pub fn pop(&mut self) {
        self.stack_queue.pop();
    }

    pub fn set_pointer(&mut self, value: usize) {
        self.pointer = value;
    }

    pub fn get_pointer(&self) -> usize {
        return self.pointer
    }

    pub fn get_value_at_pointer(&self) -> u16 {
        self.stack_queue[self.pointer]
    }
}