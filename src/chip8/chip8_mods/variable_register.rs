#[derive(Copy, Clone)]
pub struct VariableRegister {
    value: u8,
}

impl VariableRegister {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn set(&mut self, val: u8) {
        self.value = val;
    }

    pub fn get(&self) -> u8 {
        self.value
    }
}
