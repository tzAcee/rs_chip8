pub struct DelayTimer {
    value: u8
}

impl DelayTimer {
    pub fn new() -> Self {
        Self{ value: 255}
    }

    pub fn tick(&mut self) {
        if(self.value>=0) {
            self.value-=1;
        } else {
            println!("Delay Timer reached 0");
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}