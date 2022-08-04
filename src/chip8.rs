mod chip8_mods;
use chip8_mods::*;
use std::{thread, time};

pub struct Chip8 {
    memory: memory::Memory,
    display: display::Display,
    pc: program_counter::ProgramCounter,
    i: index_register::IndexRegister,
    stack: stack::Stack,
    delay_timer: delay_timer::DelayTimer,
    sound_timer: sound_timer::SoundTimer,
    variable_registers: [variable_register::VariableRegister; 16]
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: memory::Memory::new(),
            display: display::Display::new(),
            pc: program_counter::ProgramCounter{},
            i: index_register::IndexRegister{},
            stack: stack::Stack::new(),
            delay_timer: delay_timer::DelayTimer::new(),
            sound_timer: sound_timer::SoundTimer::new(),
            variable_registers: [variable_register::VariableRegister{}; 16]
        }
    }

    pub fn run(&self, timeout_sec: u64) {
        self.fetch();
        self.decode();
        self.execute();

        thread::sleep(time::Duration::from_secs(timeout_sec));
    }

    fn fetch(&self) {

    }
    fn decode(&self) {

    }
    fn execute(&self) {
        
    }
}














