use crate::chip8::chip8_mods::memory;

pub struct ProgramCounter {
    points_at: u32
}

impl ProgramCounter {
    pub fn new() -> Self {
        Self {points_at: 0}
    }

    pub fn get_point_value(&self) -> u32 {
        self.points_at
    }

    pub fn set_point_value(&mut self, val: u32) {
        const memory_size: u32 = 4096; // KB
                                        // instruction is 2 bytes long
        if val <= 4094 {
            self.points_at = val;
        } else {
            panic!("INVALID PROGRAM COUNTER POINT VALUE!");
        }
    }
}