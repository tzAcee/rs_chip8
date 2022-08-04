pub struct IndexRegister {
    value: u16,
}

impl IndexRegister {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn set(&mut self, val: u16) {
        self.value = val;
    }

    pub fn get(&self) -> u16 {
        self.value
    }
}
