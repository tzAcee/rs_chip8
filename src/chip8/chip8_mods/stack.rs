use std::collections::VecDeque;

pub struct Stack {
    stack_queue: Vec<u16>
}

impl Stack {
    pub fn new() -> Self {
        Self { stack_queue: Vec::new()}
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
}