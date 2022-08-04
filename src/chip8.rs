mod chip8_mods;
use chip8_mods::*;
use std::{
    thread::{self, current},
    time,
};

pub struct Chip8 {
    memory: memory::Memory,
    display: display::Display,
    pc: program_counter::ProgramCounter,
    i: index_register::IndexRegister,
    stack: stack::Stack,
    delay_timer: delay_timer::DelayTimer,
    sound_timer: sound_timer::SoundTimer,
    variable_registers: [variable_register::VariableRegister; 16],
    current_instruction: u16,
    current_function: fn(&mut Chip8),
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: memory::Memory::new(),
            display: display::Display::new(),
            pc: program_counter::ProgramCounter::new(),
            i: index_register::IndexRegister {},
            stack: stack::Stack::new(),
            delay_timer: delay_timer::DelayTimer::new(),
            sound_timer: sound_timer::SoundTimer::new(),
            variable_registers: [variable_register::VariableRegister::new(); 16],
            current_instruction: 0,
            current_function: { |this| () },
        }
    }

    pub fn run(&mut self, timeout_sec: u64) {
        loop {
            self.fetch();
            self.decode();
            self.execute();

            thread::sleep(time::Duration::from_secs(timeout_sec));
        }
    }

    fn fetch(&mut self) {
        self.current_instruction = self.memory.get_instruction(self.pc.get_point_value());
        self.pc.set_point_value(self.pc.get_point_value() + 2);
    }
    fn decode(&mut self) {
        match (self.current_instruction & 0xF000) {
            0x0 => {
                // 00E0 Clear Screen
                self.current_function = |this| (*this).display.clear();
            }
            0x1 => {
                // 1NNN Jump to NNN
                self.current_function = |this| {
                    let address = ((*this).current_instruction & 0x0FFF) as u32;
                    (*this).pc.set_point_value(address);
                }
            }
            0x2 => {}
            0x3 => {}
            0x4 => {}
            0x5 => {}
            0x6 => {
                // 6XNN // set register VX to NN
                self.current_function = |this| {
                    let NN = ((*this).current_instruction & 0x00FF) as u8;
                    let X = ((*this).current_instruction & 0x0F00) >> 8;
                    (*this).variable_registers[X as usize].set(NN);
                };
            }
            0x7 => {
                // 7XNN (add value to register VX)
                self.current_function = |this| {
                    let NN = ((*this).current_instruction & 0x00FF) as u8;
                    let X = ((*this).current_instruction & 0x0F00) >> 8;
                    let current_vx = (*this).variable_registers[X as usize].get();
                    (*this).variable_registers[X as usize].set(NN + current_vx);
                }
            }
            0x8 => {}
            0x9 => {}
            0xA => {
                // ANNN (set index register I)
                self.current_function = |this| {
                    let NN = ((*this).current_instruction & 0x0FFF) as u8;
                    this.i.set(NN);
                }
            }
            0xB => {}
            0xC => {}
            0xD => {}
            0xE => {}
            0xF => {}
            _ => {
                println!("Did not found Nibble of OPCODE.");
            }
        }
    }
    fn execute(&mut self) {
        (self.current_function)(self);
    }
}
