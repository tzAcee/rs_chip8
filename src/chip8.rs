mod chip8_mods;
use chip8_mods::*;

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
            display: display::Display{},
            pc: program_counter::ProgramCounter{},
            i: index_register::IndexRegister{},
            stack: stack::Stack{},
            delay_timer: delay_timer::DelayTimer{},
            sound_timer: sound_timer::SoundTimer{},
            variable_registers: [variable_register::VariableRegister{}; 16]
        }
    }
}














